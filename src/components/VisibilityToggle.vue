<script setup lang="ts">
const appStore = useAppStore()
</script>

<template>
    <UTooltip :text="`Showing ${10} of ${31} available nodes`">
        <UButton
            variant="ghost"
            color="neutral"
            @click="appStore.toggleHiddenNodeVisibility"
            :aria-pressed="appStore.showHiddenNodes.toString()"
            :icon="appStore.showHiddenNodes ? 'i-lucide-eye' : 'i-lucide-eye-off'"
        >
        <span v-if="appStore.showGroups">
            {{ appStore.filteredNodeGroups.length }} / {{ appStore.nodeGroups.length }}
        </span>
        <span v-else>
            {{ appStore.filteredNodes.length }} / {{ appStore.nodes.length }}
        </span>
        </UButton>
    </UTooltip>
</template>
