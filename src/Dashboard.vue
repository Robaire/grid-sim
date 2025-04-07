<script setup lang="ts">
import Button from 'primevue/button';
import Toolbar from 'primevue/toolbar';
import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';
import Skeleton from 'primevue/skeleton';

import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';

async function loadModel() {
    const selectedFile = await open({ multiple: false });

    if (selectedFile) {
        await invoke("load_model", { path: selectedFile });
    }
}

async function saveModel() {
    const selectedFile = await save({ defaultPath: 'model.json' });

    if (selectedFile) {
        console.log(selectedFile);
        await invoke("save_model", { path: selectedFile });
    }
}

</script>
<template>

    <Toolbar class="w-full">
        <template #start>
            <Button icon="pi pi-play" label="Run Model" severity="secondary" text />
        </template>
        <template #end>
            <Button icon="pi pi-save" label="Save" class="mr-2" severity="secondary" text @click="saveModel" />
            <Button icon="pi pi-upload" label="Load" class="mr-2" severity="secondary" text @click="loadModel" />
        </template>
    </Toolbar>

    <TabView class="w-full">
        <TabPanel header="Tab 1" value="tab1">
            <Skeleton width="100%" height="30rem"></Skeleton>
        </TabPanel>
        <TabPanel header="Tab 2" value="tab2">
            <Skeleton width="100%" height="30rem"></Skeleton>
        </TabPanel>
        <TabPanel header="Tab 3" value="tab3">
            <Skeleton width="100%" height="30rem"></Skeleton>
        </TabPanel>
    </TabView>


</template>