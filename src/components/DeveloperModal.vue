<script setup lang="ts">
const props = defineProps({
    modelValue: {
        type: Boolean,
        required: true,
    },
});

const emit = defineEmits(["update:modelValue"]);

const isOpen = computed({
    get: () => props.modelValue,
    set: (val) => emit("update:modelValue", val),
});

const appStore = useAppStore()

</script>

<template>
    <UModal
        v-model:open="isOpen"
        title="Debug & Diagnostics"
        description="A peek behind the scenes—inspect presets, config, and more."
    >
        <template #body>
            <div class="space-y-6">
                <!-- System Info -->
                <UCard variant="soft">
                    <template #header>
                        <div class="inline-flex items-center gap-2">
                            <UIcon name="i-lucide-monitor-cog"/>
                            <div class="font-medium">System Info</div>
                        </div>
                    </template>

                    <table class="text-sm">
                        <tbody>
                            <tr>
                                <td class="pr-4 py-0.5 text-(--ui-text-muted)">Platform:</td>
                                <td class="px-4 py-0.5 break-all whitespace-normal max-w-xs w-full">
                                    {{ appStore.platform }}
                                </td>
                            </tr>
                            <tr>
                                <td class="pr-4 py-0.5 text-(--ui-text-muted)">Username:</td>
                                <td class="px-4 py-0.5 break-all whitespace-normal max-w-xs w-full">
                                    {{ appStore.username }}
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </UCard>

                <UCard variant="soft">
                    <template #header>
                        <div class="inline-flex items-center gap-2">
                            <UIcon name="i-lucide-git-branch"/>
                            <div class="font-medium">Selected EnvPreset</div>
                        </div>
                    </template>

                    <div v-if="appStore.selectedEnvPreset">
                        <p class="font-semibold text-sm align-top">{{ appStore.selectedEnvPreset.name }}</p>
                        <p class="text-sm text-(--ui-text-muted)">
                            {{ appStore.selectedEnvPreset.description }}
                        </p>
                        <p class="text-xs italic break-all text-(--ui-text-dimmed) mb-2">
                            {{ appStore.selectedEnvPreset.filepath }}
                        </p>

                        <USeparator class="py-1"/>

                        <table class="text-sm">
                            <tbody>
                                <tr v-for="env in appStore.selectedEnvPreset.env">
                                    <td class="pr-4 py-0.5 text-(--ui-text-muted) align-top">{{ env.name }}:</td>
                                    <td class="px-4 py-0.5 break-all whitespace-normal max-w-xs w-full">
                                        {{ env.value }}
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div v-else class="text-sm italic text-(--ui-text-muted)">No preset selected</div>
                </UCard>

                <!-- App Config Section -->
                <UCard variant="soft">
                    <template #header>
                        <div class="inline-flex items-center gap-2">
                            <UIcon name="i-lucide-cog"/>
                            <div class="font-medium">Configuration</div>
                        </div>
                    </template>

                    <pre class="text-xs rounded p-3 overflow-auto">{{ JSON.stringify(appStore.appConfig, null, 2) }}</pre>
                </UCard>
            </div>
        </template>
    </UModal>
</template>
