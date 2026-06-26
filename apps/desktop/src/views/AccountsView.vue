<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BaseCard from '../components/BaseCard.vue';

const selectedIcon = ref('');
const submittedName = ref('');
const selectedType = ref('');
const submittedBalance = ref('');

async function createAccount() {
    await invoke('create_account', {
        name: submittedName,
        iconKey: selectedIcon,
        accountType: selectedType
    })
}

async function submitBudget() {
    await invoke('')
}
</script>

<template>
    <div>
        <BaseCard>
            <button @click="invoke('vue')">Invoke</button>
            <button></button>
        </BaseCard>
    </div>
    <div></div>
    <div></div>
</template>