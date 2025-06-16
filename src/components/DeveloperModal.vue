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

const nodeStore = useVaroNodeStore();

const tableData = ref([
    { key: "API_URL", value: "https://api.example.com" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    {
        key: "DEBUG_MODE",
        value: "C:/Users/joker/Documents/GitHub/data-led-designs/pivot_tables/Master Gatlinburg Spreadsheet - Dec 2024 - Copy.xlsx",
    },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "DEBUG_MODE", value: "true" },
    { key: "TIMEOUT", value: "3000ms" },
]);


const columns = [
  { key: 'label', label: 'Field', sortable: false },
  { key: 'value', label: 'Value', sortable: false }
]

const rows = [
  { label: 'Username', value: 'john.super.long.email+dev@example-reallylongdomain.com' },
  { label: 'Platform', value: 'win32-x64 10.0.19045 (Windows 10)' }
]

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
                        <div class="font-medium">System Info</div>
                    </template>

                    <table>
                        <tbody>
                            <tr>
                                <td class="pr-4">Platform</td>
                                <td class="px-4 break-all whitespace-normal max-w-xs w-full">
                                    win
                                </td>
                            </tr>
                            <tr>
                                <td class="pr-4 py-1">Username</td>
                                <td class="px-4 py-1 break-all whitespace-normal max-w-xs w-full">
                                    joker
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </UCard>

                <UCard variant="soft">
                    <template #header>
                        <div class="font-medium">Selected EnvPreset</div>
                    </template>

                    <div v-if="nodeStore.selectedEnvPreset">
                        <p class="font-semibold text-base">{{ nodeStore.selectedEnvPreset.name }}</p>
                        <p class="text-sm text-(--ui-text-dimmed) mb-2">
                            {{ nodeStore.selectedEnvPreset.description }}
                        </p>
                        <UTable
                            sticky
                            :data="nodeStore.selectedEnvPreset.env"
                            class="flex-1 max-h-1/2"
                            :ui="{
                                td: 'py-1.5',
                                th: 'py-1.5',
                            }"
                        />
                        <!-- <ul class="text-sm space-y-1">
                    <li
                    v-for="env in nodeStore.selectedEnvPreset.env"
                    :key="env.name"
                    class="flex justify-between border-b pb-1"
                    >
                    <span><code>{{ env.name }}</code></span>
                    <span>{{ env.value }} <span class="text-gray-400">[{{ env.operation }}]</span></span>
                    </li>
                </ul> -->
                    </div>
                    <div v-else class="text-sm italic text-gray-400">No preset selected</div>
                </UCard>

                <!-- App Config Section -->
                <UCard variant="soft">
                    <template #header>
                        <div class="font-medium">Configuration</div>
                    </template>

                    <pre class="text-xs rounded p-3 overflow-auto max-h-64">
            <!-- {{ JSON.stringify(config, null, 2) }} -->
        </pre>
                </UCard>
            </div>
        </template>
    </UModal>
</template>
