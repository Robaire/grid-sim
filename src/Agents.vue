<script setup lang="ts">
import { ref } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import Menu from 'primevue/menu'
import InputText from 'primevue/inputtext'
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const AgentTypes = {
    CUSTOMER: 'Customer',
    POWERPLANT: 'Power Plant',
    BATTERYSTORAGE: 'Battery Storage',
}

// Define type from the values of AgentTypes
type AgentType = typeof AgentTypes[keyof typeof AgentTypes]

interface Agent {
    id: string
    name: string
    agentType: AgentType
}

const agentLabels = ref([
    { label: AgentTypes.POWERPLANT, icon: 'pi pi-building', command: () => { addAgent(AgentTypes.POWERPLANT) } },
    { label: AgentTypes.CUSTOMER, icon: 'pi pi-home', command: () => { addAgent(AgentTypes.CUSTOMER) } },
    { label: AgentTypes.BATTERYSTORAGE, icon: 'pi pi-bolt', command: () => { addAgent(AgentTypes.BATTERYSTORAGE) } },
])


const selectedAgent = ref<Agent | null>(null)
const agents = ref<Agent[]>([])

const getAgents = async () => {
    agents.value = (await invoke('get_agents')).map((agent) => {
        return {
            id: agent.id,
            name: agent.name,
            agentType: agent.agent_type as AgentType
        }
    });
}

const addAgent = async (agentType: AgentType) => {
    await invoke('add_agent', { agentType })
    getAgents()
}

const deleteAgent = async () => {
    const id = selectedAgent.value?.id
    if (id !== undefined) {
        await invoke('delete_agent', { id })
        getAgents()
        selectedAgent.value = null
    }
}

const updateAgent = async () => {

    const agent = {
        id: selectedAgent.value?.id,
        name: selectedAgent.value?.name,
        agent_type: selectedAgent.value?.agentType
    }

    if (selectedAgent.value !== null) {
        await invoke('set_agent', { data: agent })
    }

    getAgents()
}

onMounted(async () => { getAgents() })

const addAgentMenu = ref()
const toggleMenu = (event: Event) => {
    addAgentMenu.value?.toggle(event)
}
</script>
<template>

    <div class="flex justify-between items-center">
        <h1 class="text-2xl font-bold p-4"> {{ agents.length }} Agents</h1>
        <Button label="Add Agent" icon="pi pi-angle-down" iconPos="right" class="p-4" severity="secondary"
            @click="toggleMenu" />
        <Menu ref="addAgentMenu" :model="agentLabels" :popup="true" />
    </div>

    <DataTable v-if="agents.length > 0" v-model:selection="selectedAgent" size="small" :value="agents"
        selectionMode="single" dataKey="id" scrollable scrollHeight="300px">
        <Column field="id" header="ID">
            <template #body="{ data }">{{ data.id.toString(16).toUpperCase().substring(0, 8) }}</template>
        </Column>
        <Column field="name" header="Name"></Column>
        <Column field="agentType" header="Type">
        </Column>
    </DataTable>
    <div v-else class="text-center p-4">
        No Agents
    </div>

    <h1 class="text-2xl font-bold p-4">Configuration</h1>


    <div v-if="selectedAgent !== null">
        <div>
            <p>ID: {{ selectedAgent.id }}</p>

            <p> Name:
                <InputText v-model="selectedAgent.name" @blur="updateAgent" />
            </p>
            <p>Type: {{ selectedAgent.agentType }}</p>
        </div>
        <Button label="Delete Agent" size="small" severity="danger" @click="deleteAgent" />
    </div>
    <div v-else class="text-center p-2">Select an Agent to view its configuration</div>

</template>

<style scoped>
h1 {
    color: var(--primary-color);
}
</style>