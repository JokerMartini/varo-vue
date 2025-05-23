<script setup lang="ts">
import { computed } from "vue";
import { colord } from "colord";

interface Status {
    name: string;
    color: string;
}

const props = defineProps<{
    status: Status | null;
}>();

const badgeStyle = computed(() => {
  const rawColor = props.status?.color ?? '#000'
  const base = colord(rawColor)

  if (!base.isValid()) {
    return {
      backgroundColor: '#0000001A',
      color: '#000',
      '--tw-ring-color': '#000',
    }
  }

  return {
    backgroundColor: base.alpha(0.1).toRgbString(),
    color: base.toHex(),
    '--tw-ring-color': base.alpha(0.25).toRgbString(),
  }
})
</script>

<template>
    <UBadge v-if="status" size="sm" class="ring ring-inset" :style="badgeStyle">
        {{ status.name }}
    </UBadge>
</template>
