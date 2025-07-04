<script setup lang="ts">

const appStore = useAppStore()

// Handle preset selection
async function handlePresetChange(presetId: string | null) {
    if (presetId && presetId !== appStore.selectedEnvPresetId) {
        await appStore.selectEnvPreset(presetId)
    } else if (!presetId || presetId === "") {
        await appStore.clearEnvPreset()
    }
}

const envMenuItems = computed(() => [
    [
        {
            label: "Clear",
            icon: "i-lucide-x",
            async onSelect(e: Event) {
                await appStore.clearEnvPreset()
            },
        },
    ]
])
</script>

<template>
    <div>
        <UContextMenu :items="envMenuItems">
            <UTooltip text="Environment selector">
                <USelect
                    :model-value="appStore.selectedEnvPresetId"
                    @update:model-value="handlePresetChange"
                    :items="appStore.envPresets"
                    label-key="name"
                    value-key="id"
                    variant="soft"
                    icon="i-lucide-git-branch"
                    placeholder="Select env"
                    class="w-full min-w-36"
                />
            </UTooltip>
        </UContextMenu>
    </div>
</template>