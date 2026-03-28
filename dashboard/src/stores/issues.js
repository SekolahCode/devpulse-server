import axios from "axios";
import { defineStore } from "pinia";

const PER_PAGE = 25;

export const useIssuesStore = defineStore("issues", {
  state: () => ({
    issues:     [],
    total:      0,
    page:       1,
    perPage:    PER_PAGE,
    loading:    true,
    stats:      null,
    live:       [],
  }),

  getters: {
    totalPages: (state) => Math.max(1, Math.ceil(state.total / state.perPage)),
  },

  actions: {
    async fetch(projectId, status = "unresolved", { search = "", environment = "", release = "", page = 1 } = {}) {
      this.loading = true;
      this.page    = page;

      try {
        const { data } = await axios.get("/api/issues", {
          params: {
            project_id:  projectId,
            status,
            search:      search       || undefined,
            environment: environment  || undefined,
            release:     release      || undefined,
            limit:       this.perPage,
            offset:      (page - 1) * this.perPage,
          },
        });

        this.issues = data.data;
        this.total  = data.total ?? data.data.length;
      } finally {
        this.loading = false;
      }
    },

    async fetchStats() {
      try {
        const { data } = await axios.get("/api/stats");
        this.stats = data;
      } catch {
        // non-critical
      }
    },

    async resolve(id) {
      await axios.patch(`/api/issues/${id}`, { status: "resolved" });
      this.issues = this.issues.filter((i) => i.id !== id);
      this.total  = Math.max(0, this.total - 1);
    },

    async ignore(id) {
      await axios.patch(`/api/issues/${id}`, { status: "ignored" });
      this.issues = this.issues.filter((i) => i.id !== id);
      this.total  = Math.max(0, this.total - 1);
    },

    async bulkResolve(ids) {
      await Promise.all(ids.map(id => axios.patch(`/api/issues/${id}`, { status: "resolved" })));
      this.issues = this.issues.filter((i) => !ids.includes(i.id));
      this.total  = Math.max(0, this.total - ids.length);
    },

    async bulkIgnore(ids) {
      await Promise.all(ids.map(id => axios.patch(`/api/issues/${id}`, { status: "ignored" })));
      this.issues = this.issues.filter((i) => !ids.includes(i.id));
      this.total  = Math.max(0, this.total - ids.length);
    },

    addLiveEvent(event) {
      this.live.unshift(event);
      if (this.live.length > 20) this.live.pop();

      if (event.is_new && this.page === 1 && !this.issues.some((i) => i.id === event.issue_id)) {
        this.issues.unshift({
          id:          event.issue_id,
          title:       event.title,
          level:       event.level,
          status:      "unresolved",
          event_count: 1,
          last_seen:   new Date().toISOString(),
        });
        // Trim to page size so the row count stays consistent
        if (this.issues.length > this.perPage) this.issues.pop();
        this.total += 1;
      }
    },
  },
});
