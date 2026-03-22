import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/login",                   component: () => import("../views/LoginView.vue"),   meta: { public: true } },
    { path: "/",                        component: () => import("../views/ProjectsView.vue") },
    { path: "/projects/:id/issues",     component: () => import("../views/IssuesView.vue") },
    { path: "/projects/:id/releases",   component: () => import("../views/ReleasesView.vue") },
    { path: "/issues/:id",              component: () => import("../views/IssueDetails.vue") },
    { path: "/:pathMatch(.*)*",         redirect: "/" },
  ],
});

// Navigation guard: redirect to /login only when a token is present in storage
// but the server returns 401 (handled by axios interceptor).
// We also guard directly here: if no token is stored at all, go to login.
router.beforeEach((to) => {
  if (to.meta.public) return true          // login page is always accessible

  const token = localStorage.getItem('devpulse_token')
  if (!token) {
    return { path: '/login', query: { redirect: to.fullPath } }
  }
  return true
})

export default router
