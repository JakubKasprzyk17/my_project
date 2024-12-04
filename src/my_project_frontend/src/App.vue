<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let chat= ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const newMsg = target.querySelector('#newMsg').value;
  await my_project_backend.add_msg(newMsg);
  await getChat();
}

async function getChat() {
  chat.value =  await my_project_backend.get_chat();
}

getChat();
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="newMsg">Enter your newMsg: &nbsp;</label>
      <input id="newMsg" alt="newMsg" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <button @click = "getChat">refresh</button>
    <section id="chat">
      <div v-for="msg in chat ">{{ msg }}</div>
    </section>
  </main>
</template>