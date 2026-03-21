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
    live:        [],  // real-time events from WebSocket
  }),

  actions: {
    async fetch(projectId, status = "unresolved", { search = "", reset = true } = {}) {
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
            project_id: projectId,
            status,
            search:     search || undefined,
            limit:      50,
            offset:     reset ? 0 : this.offset,
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

    async fetchMore(projectId, status, search) {
      if (!this.hasMore || this.loadingMore) return;
      await this.fetch(projectId, status, { search, reset: false });
    },

    async fetchStats() {
      try {
        const { data } = await axios.get("/api/stats");
        this.stats = data;
      } catch {
        // stats are non-critical — fail silently
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

    // Called by WebSocket when a new event arrives
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
