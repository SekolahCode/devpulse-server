<script setup>
import { cn } from '@/lib/utils'
import { DropdownMenuSeparator } from 'reka-ui'

const props = defineProps({ class: { type: null, default: undefined } })
</script>

<template>
  <DropdownMenuSeparator
    data-slot="dropdown-menu-separator"
    :class="cn('my-1.5 h-px bg-[var(--dp-rule)]', props.class)"
  />
</template>
