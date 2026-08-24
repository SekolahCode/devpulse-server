<script setup>
import { cn } from '@/lib/utils'
import { Check, Minus } from 'lucide-vue-next'
import { CheckboxIndicator, CheckboxRoot } from 'reka-ui'

// reka-ui's CheckboxRoot models via `modelValue`/`update:modelValue` (this
// version renamed the API from the older `checked`/`update:checked`) — this
// component keeps the `checked` naming at its own boundary since that's the
// clearer name for callers, and translates to modelValue underneath.
const props = defineProps({
  checked: { type: [Boolean, String], default: false },
  disabled: { type: Boolean, default: false },
  class: { type: null, default: undefined },
})
const emit = defineEmits(['update:checked'])
</script>

<template>
  <CheckboxRoot
    data-slot="checkbox"
    :model-value="checked"
    :disabled="disabled"
    @update:model-value="emit('update:checked', $event)"
    :class="cn(
      'peer size-4 shrink-0 rounded-[4px] border border-[var(--dp-rule)] shadow-xs transition-colors outline-none',
      'data-[state=checked]:bg-[var(--dp-accent)] data-[state=checked]:border-[var(--dp-accent)] data-[state=checked]:text-white',
      'data-[state=indeterminate]:bg-[var(--dp-accent)] data-[state=indeterminate]:border-[var(--dp-accent)] data-[state=indeterminate]:text-white',
      'focus-visible:ring-2 focus-visible:ring-[var(--dp-accent)]/30',
      'disabled:cursor-not-allowed disabled:opacity-50',
      props.class,
    )"
  >
    <CheckboxIndicator
      data-slot="checkbox-indicator"
      class="flex items-center justify-center text-current transition-none"
    >
      <slot>
        <Minus v-if="checked === 'indeterminate'" class="size-3.5" />
        <Check v-else class="size-3.5" />
      </slot>
    </CheckboxIndicator>
  </CheckboxRoot>
</template>
