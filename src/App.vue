<script setup lang="ts">
import { ref } from 'vue'
import Dashboard from './Dashboard.vue'
import Agents from './Agents.vue'
import Settings from './Settings.vue'


const currentView = ref('dashboard')

</script>

<template>
    <div class="flex h-screen">
        <!-- Sidebar -->
        <aside class="w-48 bg-gray-800 text-white p-4">
            <h2 class="text-xl font-bold">EM426</h2>
            <ul class="mt-4 space-y-2">
                <li>
                    <a href="#" @click.prevent="currentView = 'dashboard'"
                        :class="['block p-2 hover:bg-gray-700', currentView === 'home' ? 'bg-gray-700' : '']">
                        Dashboard
                    </a>
                </li>
                <li>
                    <a href="#" @click.prevent="currentView = 'agents'"
                        :class="['block p-2 hover:bg-gray-700', currentView === 'agents' ? 'bg-gray-700' : '']">
                        Agents
                    </a>
                </li>
                <li>
                    <a href="#" @click.prevent="currentView = 'settings'"
                        :class="['block p-2 hover:bg-gray-700', currentView === 'settings' ? 'bg-gray-700' : '']">
                        Settings
                    </a>
                </li>
            </ul>
        </aside>

        <!-- Main Content -->
        <main class="flex-1 p-6">
            <div v-if="currentView === 'dashboard'">
                <Dashboard />
            </div>
            <div v-else-if="currentView === 'agents'">
                <Agents />
            </div>
            <div v-else-if="currentView === 'settings'">
                <Settings />
            </div>
        </main>
    </div>
</template>

<style>
@import "tailwindcss";
@import "tailwindcss-primeui";
@import 'primeicons/primeicons.css';

:root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}

html,
body {
    overflow: hidden;
    height: 100%;
}
</style>
