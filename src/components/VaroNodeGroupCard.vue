<script setup lang="ts">
import type { VaroNodeGroup, } from '@/types/VaroNode';

const props = defineProps<{ node: VaroNodeGroup }>();

const items = ref<DropdownMenuItem[]>([
    {
        label: 'Hide',
        icon: 'i-lucide-eye-off'
    },
    {
        label: 'Edit',
        icon: 'i-lucide-pencil'
    }
])

</script>

<template>
    <div class="bg-(--ui-bg-elevated)/50 p-2.5 rounded-[calc(var(--ui-radius)*2)] relative overflow-hidden">
        Here
    </div>
</template>


<!-- card horizontal / vertical -->