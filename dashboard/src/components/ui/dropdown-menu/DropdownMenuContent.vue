<script setup>
import { cn } from '@/lib/utils'
import { DropdownMenuContent, DropdownMenuPortal, useForwardPropsEmits } from 'reka-ui'
import { computed } from 'vue'

// DropdownMenuPortal is the whole point of this component: it teleports the
// menu to <body>, so it floats above any ancestor with `overflow-hidden`
// (e.g. the boxed table rows) instead of being clipped by it.
const props = defineProps({
  sideOffset: { type: Number, default: 6 },
  align: { type: String, default: 'end' },
  class: { type: null, default: undefined },
})
const emits = defineEmits(['escapeKeyDown', 'pointerDownOutside', 'focusOutside', 'interactOutside'])

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props
  return delegated
})

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <DropdownMenuPortal>
    <DropdownMenuContent
      data-slot="dropdown-menu-content"
      v-bind="forwarded"
      :class="cn(
        'dv-dropdown z-50 min-w-[10rem] overflow-hidden rounded-xl border border-[var(--dp-rule)] bg-[var(--dp-surface)] py-1.5 text-left',
        props.class,
      )"
    >
      <slot />
    </DropdownMenuContent>
  </DropdownMenuPortal>
</template>

<style scoped>
.dv-dropdown {
  box-shadow: 0 8px 24px oklch(22% 0.02 258 / 14%);
}
</style>
