import { defineStore } from "pinia";
import { VaroNode } from "~/models/VaroNode";
import { VaroNodeGroup } from "~/models/VaroNodeGroup";
import { VaroCategory } from "~/models/VaroCategory";
import { EnvPreset } from "~/models/EnvPreset";
import { getNodeGroupsByCategory, getCategoriesFromNodes } from "~/utils/nodeGrouping";
import { invoke } from "@tauri-apps/api/core";

export const useAppStore = defineStore("app", () => {
    // UI State
    const showHiddenNodes = ref(false);
    const showGroups = ref(true);
    const showCategories = ref(false);
    const showAboutDialog = ref(false);
    const showDeveloperDialog = ref(false);
    const searchQuery = ref("");
    const loading = ref(false);
    
    // Backend Data
    const nodes = ref<VaroNode[]>([]);
    const nodeGroups = ref<VaroNodeGroup[]>([]);
    const categories = ref<VaroCategory[]>([]);
    const envPresets = ref<EnvPreset[]>([]);
    const selectedEnvPresetId = ref<string | null>(null);
    const appConfig = ref<any>(null);
    
    // System Info
    const username = ref<string | null>(null);
    const platform = ref<string | null>(null);

    // COMPUTED PROPERTIES
    const filteredNodes = computed(() => {
        const query = searchQuery.value.trim().toLowerCase();

        return nodes.value.filter((node) => {
            const matchesSearch =
                !query || node.name.toLowerCase().includes(query) || node.description?.toLowerCase().includes(query);
            const isVisible = showHiddenNodes.value || node.visible;
            return matchesSearch && isVisible;
        });
    });

    const filteredNodeGroups = computed(() => {
        const query = searchQuery.value.trim().toLowerCase();

        return nodeGroups.value.filter((group) => {
            return group.nodes.some((node) => {
                const matchesSearch = !query || node.name.toLowerCase().includes(query);
                const isVisible = showHiddenNodes.value || group.visible;
                return matchesSearch && isVisible;
            });
        });
    });

    const filteredCategories = computed(() => {
        // Reset categories
        for (const category of categories.value) {
            category.nodes = [];
            category.groups = [];
        }

        // Populate filtered data
        for (const category of categories.value) {
            const categoryName = category.name;
            category.nodes = filteredNodes.value.filter((n) => n.category === categoryName);
            category.groups = filteredNodeGroups.value.filter((n) => n.category === categoryName);
        }

        return categories.value;
    });

    const selectedEnvPreset = computed(() => {
        return envPresets.value.find(preset => preset.id === selectedEnvPresetId?.value);
    });

    // UI METHODS
    function toggleHiddenNodeVisibility() {
        showHiddenNodes.value = !showHiddenNodes.value;
    }

    function toggleGroups() {
        showGroups.value = !showGroups.value;
    }

    function toggleCategories() {
        showCategories.value = !showCategories.value;
    }

    function unhideAllNodes() {
        nodes.value.forEach((node) => {
            node.visible = true;
        });
    }

    function unhideAllNodeGroups() {
        nodeGroups.value.forEach((group) => {
            group.visible = true;
        });
    }

    // BACKEND DATA FETCHING
    async function fetchNodes() {
        try {
            loading.value = true;
            const result = await invoke<any[]>("get_nodes");
            if (Array.isArray(result)) {
                const fetchedNodes = result.map((nodeData: any) => new VaroNode(nodeData));
                setNodes(fetchedNodes);
                console.log("Loaded nodes:", fetchedNodes.length);
            } else {
                console.warn("Unexpected nodes response format:", result);
            }
        } catch (error) {
            console.error("Failed to fetch nodes:", error);
        } finally {
            loading.value = false;
        }
    }

    async function fetchEnvPresets() {
        try {
            const result = await invoke<any[]>("get_env_presets");
            if (Array.isArray(result)) {
                envPresets.value = result.map((preset: any) => new EnvPreset(preset));
                console.log("Loaded env presets:", envPresets.value.length);
            } else {
                console.warn("Unexpected env presets response format:", result);
            }
        } catch (error) {
            console.error("Failed to fetch env presets:", error);
        }
    }

    async function fetchSystemInfo() {
        try {
            username.value = await invoke<string>("get_os_username");
            platform.value = await invoke<string>("get_platform");
            console.log(`System info - User: ${username.value}, Platform: ${platform.value}`);
        } catch (error) {
            console.error("Failed to fetch system info:", error);
        }
    }

    async function fetchAppConfig() {
        try {
            const result = await invoke("get_config");
            appConfig.value = result;
            console.log("App config loaded");
        } catch (error) {
            console.error("Failed to fetch app config:", error);
        }
    }

    // ENVIRONMENT PRESET MANAGEMENT
    async function selectEnvPreset(presetId: string) {
        try {
            await invoke("select_env_preset", { id: presetId });
            selectedEnvPresetId.value = presetId;
            
            // Reload nodes after preset change
            await fetchNodes();
            console.log(`Selected env preset: ${presetId}`);
        } catch (error) {
            console.error("Failed to select env preset:", error);
        }
    }

    // NODE EXECUTION
    async function executeNode(nodeId: string) {
        console.log(`Executing node: ${nodeId}`);
        try {
            await invoke("execute_node", { id: nodeId });
            console.log(`Executed node: ${nodeId}`);
        } catch (error) {
            console.error("Failed to execute node:", error);
        }
    }

    // NODE FOLDER
    async function showNodeInFolder(nodeId: string) {
        try {
            await invoke("show_node_in_folder", { id: nodeId });
            console.log(`Opened folder for node: ${nodeId}`);
        } catch (error) {
            console.error("Failed to show node in folder:", error);
        }
    }

    // DATA MANAGEMENT
    function setNodes(newNodes: VaroNode[]) {
        nodes.value = newNodes;
        nodeGroups.value = getNodeGroupsByCategory(newNodes);
        categories.value = getCategoriesFromNodes(newNodes);
    }

    async function initializeApp() {
        console.log("Initializing application...");
        loading.value = true;
        
        try {
            // Load all initial data in parallel where possible
            await Promise.all([
                fetchSystemInfo(),
                fetchAppConfig(),
                fetchEnvPresets(),
            ]);
            
            // Load nodes after presets are available
            await fetchNodes();
            
            console.log("Application initialized successfully");
        } catch (error) {
            console.error("Failed to initialize application:", error);
        } finally {
            loading.value = false;
        }
    }

    async function refreshData() {
        console.log("Refreshing application data...");
        loading.value = true;
        
        try {
            // Reload backend configuration
            await invoke("reload_config");
            
            // Refresh all data
            await Promise.all([
                fetchEnvPresets(),
                fetchAppConfig(),
            ]);
            
            // Reload nodes
            await fetchNodes();
            
            console.log("Data refreshed successfully");
        } catch (error) {
            console.error("Failed to refresh data:", error);
        } finally {
            loading.value = false;
        }
    }

    return {
        // State
        nodes,
        nodeGroups,
        categories,
        filteredNodes,
        filteredNodeGroups,
        filteredCategories,
        envPresets,
        selectedEnvPresetId,
        selectedEnvPreset,
        appConfig,
        username,
        platform,
        loading,
        showHiddenNodes,
        showGroups,
        showCategories,
        showAboutDialog,
        showDeveloperDialog,
        searchQuery,

        // UI Methods
        toggleHiddenNodeVisibility,
        toggleGroups,
        toggleCategories,
        unhideAllNodes,
        unhideAllNodeGroups,

        // Data Methods
        initializeApp,
        refreshData,
        fetchNodes,
        fetchEnvPresets,
        fetchSystemInfo,
        fetchAppConfig,
        selectEnvPreset,
        executeNode,
        showNodeInFolder,
        setNodes,
    };
});