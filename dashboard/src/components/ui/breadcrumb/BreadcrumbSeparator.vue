<script setup>
import { cn } from '@/lib/utils'
import { ChevronRight } from 'lucide-vue-next'

const props = defineProps({ class: { type: null, default: undefined } })
</script>

<template>
  <li
    data-slot="breadcrumb-separator"
    role="presentation"
    aria-hidden="true"
    :class="cn('[&>svg]:size-3.5 text-[var(--dp-rule)]', props.class)"
  >
    <slot>
      <ChevronRight />
    </slot>
  </li>
</template>
