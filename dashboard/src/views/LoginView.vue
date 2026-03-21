<template>
  <div class="min-h-screen bg-[#0a0a12] flex items-center justify-center px-4">
    <div class="w-full max-w-sm">

      <!-- Logo / title -->
      <div class="text-center mb-8">
        <div class="text-2xl font-bold text-white tracking-tight mb-1">DevPulse</div>
        <p class="text-sm text-gray-500">Enter your admin token to continue</p>
      </div>

      <!-- Card -->
      <form
        @submit.prevent="submit"
        class="bg-[#111119] border border-white/6 rounded-2xl px-6 py-7 space-y-5"
      >
        <div>
          <label class="block text-xs font-medium text-gray-400 mb-1.5">Admin Token</label>
          <!-- Hidden username field satisfies browser/password-manager accessibility requirements -->
          <input type="text" name="username" value="admin" autocomplete="username" aria-hidden="true" style="display:none" />
          <input
            v-model="token"
            type="password"
            placeholder="Paste your ADMIN_TOKEN here"
            autocomplete="current-password"
            class="w-full bg-[#0d0d16] border border-white/8 rounded-xl px-4 py-2.5 text-sm text-gray-200
                   placeholder-gray-600 focus:outline-none focus:border-violet-500/60 transition-colors"
          />
        </div>

        <p v-if="error" class="text-xs text-red-400">{{ error }}</p>

        <button
          type="submit"
          :disabled="!token.trim()"
          class="w-full bg-violet-600 hover:bg-violet-500 disabled:opacity-40 disabled:cursor-not-allowed
                 text-white text-sm font-medium rounded-xl py-2.5 transition-colors"
        >
          Sign in
        </button>
      </form>

      <p class="text-center text-xs text-gray-600 mt-5">
        Set <code class="text-gray-500">ADMIN_TOKEN</code> in your
        <code class="text-gray-500">.env</code> file to enable authentication.
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
