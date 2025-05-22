<script setup lang="ts">
import { VaroNode } from "~/models/VaroNode";
import { VaroNodeGroup } from "~/models/VaroNodeGroup";

const props = defineProps({
    modelValue: {
        type: Array,
        required: true,
    },
});

</script>
<template>
    <div class="p-1">
        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(180px,1fr))]">
            <template v-for="item in props.modelValue">
                <VaroNodeCard v-if="(item instanceof VaroNode)" :node="item" />
                <VaroNodeGroupCard v-else-if="(item instanceof VaroNodeGroup)" :group="item" />
            </template>
        </div>
    </div>
</template>
