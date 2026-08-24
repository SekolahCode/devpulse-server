<script setup>
import { cn } from '@/lib/utils'

const props = defineProps({ class: { type: null, default: undefined } })
</script>

<template>
  <ol
    data-slot="breadcrumb-list"
    :class="cn('flex flex-wrap items-center gap-1.5 text-[13px] text-[var(--dp-ink-3)] break-words', props.class)"
  >
    <slot />
  </ol>
</template>
