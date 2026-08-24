<script setup>
import { cn } from '@/lib/utils'

const props = defineProps({ class: { type: null, default: undefined } })
</script>

<template>
  <span
    data-slot="breadcrumb-page"
    role="link"
    aria-disabled="true"
    aria-current="page"
    :class="cn('font-medium text-[var(--dp-ink)]', props.class)"
  >
    <slot />
  </span>
</template>
