<script setup lang="ts">
import type { VaroNodeGroup } from '@/models/VaroNodeGroup';
import type { VaroNode } from '@/models/VaroNode';
import type { DropdownMenuItem } from '@nuxt/ui';

const toast = useToast()

const props = defineProps<{ group: VaroNodeGroup }>();

const nodesMenuOpen = ref(false)
const menuItems = computed<DropdownMenuItem[][]>(() => [
  [
    {
      label: 'Hide',
      disabled: !props.group.visible,
      icon: 'i-lucide-eye-off',
      onSelect(e: Event) {
            props.group.visible = false;
        }
    },
    {
      label: 'Unhide',
      disabled: props.group.visible,
      icon: 'i-lucide-eye',
      onSelect(e: Event) {
        props.group.visible = true;
      }
    }
  ],
  [
    {
      label: 'Show in Explorer...',
      icon: 'i-lucide-folder',
      onSelect(e: Event) {
        console.log('TODO');
      }
    }
  ]
]);

const groupNodeItems = computed<DropdownMenuItem[]>(() => {
  return props.group.nodes.map((node) => ({
    item: node,
    onSelect: (e: Event) => {
        props.group.selectedNodeId = node.id;
        console.log(node);
        console.log(props.group.selectedNodeId);
    }
  }));
});

function onExecuteNode(node: VaroNode) {
  console.log("Node executed:", node)
  nodesMenuOpen.value = false;
  
  toast.add({
    title: `Launching ${node?.name}`,
    description: 'Your wish is my command...',
    icon: "i-lucide-rocket",
    color: "success"
  })
}

function handleExecuteClick() {
  const selected = props.group.selectedNode
  if (selected) {
    onExecuteNode(selected)
  }
}
</script>

<template>
    <UContextMenu
        :items="menuItems"
    >
        <div 
          @click="handleExecuteClick"
          class="
            bg-(--ui-bg-elevated)/75 p-2.5 rounded-[calc(var(--ui-radius)*2)] relative overflow-hidden
            hover:bg-(--ui-bg-elevated)
            active:bg-(--ui-bg-elevated)/50
            cursor-pointer
            "
          :class="{
              'outline-dashed outline-(--ui-text-dimmed) outline-2': !group.visible
          }"
        >

            <!-- hidden -->
            <!-- <div v-if="group.visible === false"
                class="-z-100 absolute top-0 left-0 w-full h-full text-(--ui-text-dimmed)/50
                bg-[size:10px_10px] 
                bg-fixed bg-[image:repeating-linear-gradient(315deg,currentColor_0,currentColor_1px,_transparent_0,_transparent_50%)]">
            </div> -->

            <!-- content -->
            <div v-if="group.selectedNode" class="flex gap-2 items-start w-full flex-nowrap">
                <!-- Action Button/Icon -->
                <div>
                  <UIcon name="i-lucide-box" class="shrink-0 size-8"/>
                    <!-- <UButton 
                        @click="handleExecuteClick"
                        icon="i-lucide-box" 
                        variant="ghost" 
                        size="2xl" 
                        color="neutral" 
                        class="shrink-0 cursor-pointer transition transform hover:scale-105 active:scale-90 duration-100 ease-out">
                    </UButton> -->
                </div>

                <!-- Title -->
                <div class="flex-grow space-y-1 items-start">
                    <UTooltip :text="group.selectedNode.description" :disabled="!group.selectedNode.description">
                        <h3 v-if="group.nodes.length === 1" class="font-semibold text-xs">{{ group.selectedNode.name }}</h3>
                        <UDropdownMenu v-else :items="groupNodeItems" v-model:open="nodesMenuOpen">
                            <UButton  
                                @click.stop
                                trailing-icon="i-lucide-chevron-down" 
                                variant="subtle" 
                                color="neutral" 
                                class="shrink-0 text-left w-full text-xs">
                                <span class="w-full">
                                    {{ group.selectedNode.name }}
                                </span>
                            </UButton>

                            <template #item="{ item }">
                                <VaroNodeGroupOption :node="item.item" @execute="onExecuteNode"/>
                            </template>

                        </UDropdownMenu>
                    </UTooltip>

                    <!-- Badges -->
                    <div class="flex gap-1">
                        <UBadge v-if="group.selectedNode.status" 
                            class="rounded-sm" 
                            size="sm" 
                            :color="group.selectedNode.status.color" 
                            variant="subtle">
                            {{ group.selectedNode.status.name }}
                        </UBadge>
                    </div>
                </div>
            </div>
            <div v-else>
                MISSING NODE
            </div>
        </div>
    </UContextMenu>
</template>


<!-- card horizontal / vertical -->