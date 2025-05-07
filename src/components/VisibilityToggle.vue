<script setup lang="ts">
const nodeStore = useVaroNodeStore();
</script>

<template>
    <UTooltip :text="`Showing ${10} of ${31} available nodes`">
        <UButton
            variant="ghost"
            color="neutral"
            @click="nodeStore.toggleHiddenNodeVisibility"
            :aria-pressed="nodeStore.showHiddenNodes.toString()"
            :icon="nodeStore.showHiddenNodes ? 'i-lucide-eye' : 'i-lucide-eye-off'"
        >
        <span v-if="nodeStore.showGroups">
            {{ nodeStore.filteredNodeGroups.length }} / {{ nodeStore.allNodeGroups.length }}
        </span>
        <span v-else>
            {{ nodeStore.filteredNodes.length }} / {{ nodeStore.allNodes.length }}
        </span>
        </UButton>
    </UTooltip>
</template>
