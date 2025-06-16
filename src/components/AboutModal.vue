<script setup lang="ts">
import { getVersion, getName } from "@tauri-apps/api/app";

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
const appName = ref("");
const appVersion = ref("");

onMounted(async () => {
    appName.value = await getName();
    appVersion.value = await getVersion();
});
</script>

<template>
    <UModal v-model:open="isOpen" :title="`About ${appName}`" description="A stylish command center for all your digital missions.
Click, launch, conquer.">
        <template #body>
            <div class="space-y-4">
                <div class="text-sm flex flex-col gap-6">
                    <section>
                        <h4 class="text-(--ui-text-primary) leading-none font-semibold mb-2">Application</h4>
                        <p><span class="text-(--ui-text-muted)">Version:</span> {{ appVersion }}</p>
                        <p>
                            <span class="text-(--ui-text-muted)">Website:</span>
                            <a href="https://jokermartini.com" target="_blank" class="text-secondary hover:underline hover:text-(--ui-primary)"> www.JokerMartini.com</a>
                        </p>
                    </section>
                </div>
            </div>
        </template>
    </UModal>
</template>

<!-- 
<template #content>
    <div class="flex gap-2 items-center">
        <div class="flex items-center gap-2">
            <UIcon name="i-lucide-user" />
            <span>{{ nodeStore.username }}</span>
        </div>
        <div class="flex items-center gap-2">
            <UIcon name="i-lucide-laptop" />
            <span>{{ nodeStore.platform }}</span>
        </div>
    </div>
</template> -->
