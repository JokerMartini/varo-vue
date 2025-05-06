<script setup lang="ts">

const nodeStore = useVaroNodeStore()

</script>

<template>
    <UButton 
        variant="ghost" 
        color="neutral"
        @click="nodeStore.toggleHiddenNodeVisibility"
        :aria-pressed="nodeStore.showHiddenNodes.toString()"
        :icon="nodeStore.showHiddenNodes ? 'i-lucide-eye' : 'i-lucide-eye-off'" 
        >
        10/31 {{ nodeStore.showHiddenNodes }}
    </UButton>
</template>
