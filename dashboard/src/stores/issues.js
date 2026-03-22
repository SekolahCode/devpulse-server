import axios from "axios";
import { defineStore } from "pinia";

export const useIssuesStore = defineStore("issues", {
  state: () => ({
    issues:      [],
    total:       0,
    offset:      0,
    hasMore:     false,
    loading:     true,
    loadingMore: false,
    stats:       null,
    live:        [],
  }),

  actions: {
    async fetch(projectId, status = "unresolved", { search = "", environment = "", release = "", reset = true } = {}) {
      if (reset) {
        this.loading = true;
        this.offset  = 0;
        this.issues  = [];
      } else {
        this.loadingMore = true;
      }

      try {
        const { data } = await axios.get("/api/issues", {
          params: {
            project_id:  projectId,
            status,
            search:      search       || undefined,
            environment: environment  || undefined,
            release:     release      || undefined,
            limit:       50,
            offset:      reset ? 0 : this.offset,
          },
        });

        if (reset) {
          this.issues = data.data;
        } else {
          this.issues.push(...data.data);
        }

        this.total   = data.total ?? data.data.length;
        this.offset  = (reset ? 0 : this.offset) + data.data.length;
        this.hasMore = this.issues.length < this.total;
      } finally {
        this.loading     = false;
        this.loadingMore = false;
      }
    },

    async fetchMore(projectId, status, search, environment, release = "") {
      if (!this.hasMore || this.loadingMore) return;
      await this.fetch(projectId, status, { search, environment, release, reset: false });
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

      if (event.is_new && !this.issues.some((i) => i.id === event.issue_id)) {
        this.issues.unshift({
          id:          event.issue_id,
          title:       event.title,
          level:       event.level,
          status:      "unresolved",
          event_count: 1,
          last_seen:   new Date().toISOString(),
        });
        this.total += 1;
      }
    },
  },
});
