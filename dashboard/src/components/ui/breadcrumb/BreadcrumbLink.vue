<script setup>
import { cn } from '@/lib/utils'
import { Primitive } from 'reka-ui'

const props = defineProps({
  as: { type: [String, Object], default: 'a' },
  asChild: { type: Boolean, default: false },
  class: { type: null, default: undefined },
})
</script>

<template>
  <Primitive
    data-slot="breadcrumb-link"
    :as="as"
    :as-child="asChild"
    :class="cn('rounded-md px-1.5 py-0.5 -mx-1.5 transition-colors hover:text-[var(--dp-ink)] hover:bg-[var(--dp-surface-2)]', props.class)"
  >
    <slot />
  </Primitive>
</template>
