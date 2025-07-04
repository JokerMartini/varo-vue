<script setup lang="ts">

const appStore = useAppStore()

const envMenuItems = computed(() => [
    [
        {
            label: "Clear",
            icon: "i-lucide-x",
            onSelect(e: Event) {
                appStore.selectedEnvPresetId = "";
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
                    v-model="appStore.selectedEnvPresetId"
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