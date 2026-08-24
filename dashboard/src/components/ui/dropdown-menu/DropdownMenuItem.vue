<script setup>
import { cn } from '@/lib/utils'
import { DropdownMenuItem, useForwardProps } from 'reka-ui'

const props = defineProps({
  disabled: { type: Boolean, default: false },
  variant: { type: String, default: 'default' }, // 'default' | 'destructive'
  class: { type: null, default: undefined },
})

const forwardedProps = useForwardProps(props)
</script>

<template>
  <DropdownMenuItem
    data-slot="dropdown-menu-item"
    v-bind="forwardedProps"
    :class="cn(
      'flex w-full items-center gap-5 px-4 py-2.5 text-xs outline-none transition-colors cursor-pointer select-none',
      variant === 'destructive'
        ? 'text-red-600 focus:bg-red-500/10 data-[highlighted]:bg-red-500/10'
        : 'text-[var(--dp-ink-2)] focus:bg-[var(--dp-accent-soft)] focus:text-[var(--dp-accent)] data-[highlighted]:bg-[var(--dp-accent-soft)] data-[highlighted]:text-[var(--dp-accent)]',
      'data-[disabled]:pointer-events-none data-[disabled]:opacity-40',
      props.class,
    )"
  >
    <slot />
  </DropdownMenuItem>
</template>
