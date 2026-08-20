<template>
  <div class="login-root min-h-screen flex items-center justify-center px-4">
    <div class="w-full max-w-sm">

      <!-- Wordmark -->
      <div class="flex flex-col items-center mb-8">
        <div class="w-9 h-9 rounded-md bg-[var(--dp-accent)] flex items-center justify-center text-base text-white mb-3">⚡</div>
        <span class="dp-display text-[19px] text-[var(--dp-ink)] tracking-tight">DevPulse</span>
        <p class="text-sm text-[var(--dp-ink-2)] mt-1.5">Enter your admin token to continue</p>
      </div>

      <!-- Card -->
      <form
        @submit.prevent="submit"
        class="bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-md px-6 py-7 space-y-5"
      >
        <div>
          <label class="block text-xs font-medium text-[var(--dp-ink-2)] mb-1.5">Admin Token</label>
          <!-- Hidden username field satisfies browser/password-manager accessibility requirements -->
          <input type="text" name="username" value="admin" autocomplete="username" aria-hidden="true" style="display:none" />
          <input
            v-model="token"
            type="password"
            placeholder="Paste your ADMIN_TOKEN here"
            autocomplete="current-password"
            class="w-full bg-[var(--dp-paper)] border border-[var(--dp-rule)] rounded-md px-3.5 py-2.5 text-sm text-[var(--dp-ink)]
                   placeholder-[var(--dp-ink-3)] focus:outline-none focus:border-[var(--dp-accent)] focus:ring-2 focus:ring-[var(--dp-accent)]/15 transition-colors"
          />
        </div>

        <p v-if="error" class="text-xs text-[var(--dp-danger)]">{{ error }}</p>

        <button
          type="submit"
          :disabled="!token.trim()"
          class="w-full bg-[var(--dp-accent)] hover:bg-[var(--dp-accent-hover)] active:translate-y-px disabled:opacity-40 disabled:cursor-not-allowed
                 text-white text-sm font-medium rounded-md py-2.5 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--dp-accent)]"
        >
          Sign in
        </button>
      </form>

      <p class="text-center text-xs text-[var(--dp-ink-3)] mt-5">
        Set <code class="dp-mono text-[var(--dp-ink-2)]">ADMIN_TOKEN</code> in your
        <code class="dp-mono text-[var(--dp-ink-2)]">.env</code> file to enable authentication.
      </p>

    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import axios from 'axios'

const router = useRouter()
const route  = useRoute()
const token  = ref('')
const error  = ref('')

async function submit() {
  const t = token.value.trim()
  if (!t) return

  // Test the token against a protected endpoint before saving
  try {
    await axios.get('/api/stats', {
      headers: { Authorization: `Bearer ${t}` },
    })
  } catch (err) {
    if (err.response?.status === 401) {
      error.value = 'Invalid token — please check your ADMIN_TOKEN.'
    } else {
      error.value = 'Could not reach the server — please try again.'
    }
    return
  }

  // Persist and apply
  localStorage.setItem('devpulse_token', t)
  axios.defaults.headers.common['Authorization'] = `Bearer ${t}`

  const redirect = route.query.redirect || '/'
  router.push(redirect)
}
</script>

<style scoped>
/* Hallmark · macrostructure: centered auth card · genre: modern-minimal (formal/exclusive)
 * theme: DevPulse locked system (design.md) — formal white workspace, oxblood accent
 * display: Fraunces · body: Geist · outlier(mono): JetBrains Mono
 * rebuilt from a standalone hardcoded dark screen onto the shared system —
 * first thing a buyer sees, must not contradict "exclusive/formal"
 */
.login-root {
  background: var(--dp-paper);
  font-family: var(--dp-font-body);
  color: var(--dp-ink);
}

.dp-display {
  font-family: var(--dp-font-display);
  letter-spacing: -0.01em;
}

.dp-mono {
  font-family: var(--dp-font-mono);
}
</style>
