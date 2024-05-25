<template>
  <div class="loader">
    <div></div>
    <div></div>
  </div>
</template>

<style scoped lang="scss">
/* https://github.com/loadingio/css-spinner/blob/master/dist/ripple.html */

.loader {
  --loader-size: 80px;
  --loader-width: 4px;

  height: var(--loader-size);
  position: relative;
  width: var(--loader-size);
}

.loader div {
  animation: loader 1s cubic-bezier(0, 0.2, 0.8, 1) infinite;
  border: var(--loader-width) solid var(--loader-color);
  border-radius: 50%;
  opacity: 1;
  position: absolute;
}

.loader div:nth-child(2) {
  animation-delay: -0.5s;
}

@keyframes loader {
  0% {
    height: 0;
    left: calc(var(--loader-size) / 2 - var(--loader-width));
    opacity: 1;
    top: calc(var(--loader-size) / 2 - var(--loader-width));
    width: 0;
  }

  100% {
    height: calc(var(--loader-size) - var(--loader-width) * 2);
    left: 0;
    opacity: 0;
    top: 0;
    width: calc(var(--loader-size) - var(--loader-width) * 2);
  }
}
</style>
