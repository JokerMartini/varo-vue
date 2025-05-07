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

const nodeStore = useVaroNodeStore()

const tableData = ref([
  { key: 'API_URL', value: 'https://api.example.com' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'C:/Users/joker/Documents/GitHub/data-led-designs/pivot_tables/Master Gatlinburg Spreadsheet - Dec 2024 - Copy.xlsx' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'DEBUG_MODE', value: 'true' },
  { key: 'TIMEOUT', value: '3000ms' }
])

</script>

<template>
    <UModal v-model:open="isOpen" title="Env Variables" description="These are the variables powering your selected environment preset.">
        <template #body>
            <div >
                <UTable sticky :data="tableData" class="flex-1 max-h-1/2"
                :ui="{
                    td: 'py-1.5',
                    th: 'py-1.5',
                }"/>   
            </div>
        </template>
    </UModal>
</template>