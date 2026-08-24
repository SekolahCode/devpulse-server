<script setup>
import { X } from 'lucide-vue-next'
import { cn } from '@/lib/utils'

// One shell for every dialog in the app so headers, padding, footer button
// order and dismiss behaviour can't drift per-usage. Esc/click-outside are
// owned here; callers only provide `open`, a title, and the body/footer slots.
const props = defineProps({
  open:  { type: Boolean, default: false },
  title: { type: String, default: '' },
  // 'sm' for confirms and short forms, 'lg' for list-style content (alerts)
  size:  { type: String, default: 'sm' },
  class: { type: null, default: undefined },
})
defineEmits(['close'])
</script>

<template>
  <Transition name="modal">
    <div v-if="open"
      class="fixed inset-0 bg-[var(--dp-scrim)] backdrop-blur-sm flex items-center justify-center z-50 px-4"
      @click.self="$emit('close')">
      <div :class="cn(
        'dp-modal bg-[var(--dp-surface)] border border-[var(--dp-rule)] rounded-2xl w-full',
        size === 'lg' ? 'max-w-lg' : 'max-w-sm',
        props.class,
      )">
        <!-- Header -->
        <div class="flex items-start justify-between gap-4 px-6 pt-6 pb-4">
          <div class="min-w-0">
            <h2 class="text-lg font-semibold text-[var(--dp-ink)]">
              <slot name="title">{{ title }}</slot>
            </h2>
            <p v-if="$slots.description" class="text-sm text-[var(--dp-ink-2)] mt-1">
              <slot name="description" />
            </p>
          </div>
          <button @click="$emit('close')" aria-label="Close"
            class="shrink-0 -mr-1 -mt-1 w-7 h-7 flex items-center justify-center rounded-lg text-[var(--dp-ink-3)] hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)] transition-colors focus-visible:outline-2 focus-visible:outline-[var(--dp-accent)]">
            <X :size="16" />
          </button>
        </div>

        <!-- Body — footerless dialogs own the bottom padding themselves, since
             there's no action row below to supply it. -->
        <div v-if="$slots.default" class="px-6" :class="$slots.footer ? 'pb-2' : 'pb-6'">
          <slot />
        </div>

        <!-- Footer -->
        <div v-if="$slots.footer" class="flex gap-3 px-6 pt-4 pb-6">
          <slot name="footer" />
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.dp-modal {
  box-shadow: 0 12px 32px oklch(22% 0.02 258 / 16%);
}

.modal-enter-active, .modal-leave-active { transition: opacity 0.15s ease; }
.modal-enter-from, .modal-leave-to { opacity: 0; }

@media (prefers-reduced-motion: reduce) {
  .modal-enter-active, .modal-leave-active { transition-duration: 0.01s; }
}
</style>
