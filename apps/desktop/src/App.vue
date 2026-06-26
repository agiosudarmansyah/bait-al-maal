<script setup lang="ts">
import { RouterView } from 'vue-router';
import Header from './components/Header.vue';
import Navigation from './components/Navigation.vue';
</script>

<template>
  <Header />
  <Navigation />
  <main>
    <RouterView />
  </main>
</template>

<style>

html,
body {
  height: 100vh;
  width: 100vw;
  margin: 0;
  overflow: hidden;
  overscroll-behavior: none;
}

#app {
  display: grid;
  height: inherit;
  width: inherit;
  grid-template-columns: 18vw 1fr;
  grid-template-rows: 12vh 88vh;
  grid-template-areas:
    "nav header"
    "nav main";
}

main {
  padding: 2rem;
  background: var(--bg);
  border-radius: 20px 0 0 0;
  box-shadow: inset 20px 20px 60px #bebebe,
              inset -20px -20px 60px #ffffff;
  grid-area: main;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: light) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>