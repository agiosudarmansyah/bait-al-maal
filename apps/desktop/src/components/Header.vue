<script setup lang="ts">
</script>

<template>
  <header>
    <ul class="user_actions">
      <li>
        <button>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75" />
          </svg>
        </button>
      </li>
      <li>
        <button :style="{ width: '50px', height: '50px', borderRadius: '50%' }">?</button>
      </li>
    </ul>
  </header>
</template>

<style scoped>
  ul {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0;
    gap: 1rem;
  }

  li {
    display: flex;
    justify-content: center;
    align-items: center;
    white-space: nowrap;
  }

  svg {
    width: 24px;
    height: 24px;
  } 

    .user_info {
    justify-content: flex-start;
    gap: 0.5rem;
  }

  .nav_links {
    justify-content: flex-start;
    gap: 1rem;
  }

  .user_actions {
    justify-content: flex-end;
    gap: 0.5rem;
  }
</style>

<style>
 header {
  display: flex;
  flex-direction: row-reverse;
  justify-content: space-between;
  align-items: center;
  margin: 1rem;
  background: var(--bg);
  grid-area: header;
}
</style>