<template>
  <router-view :key="route.fullPath" />
</template>

<script setup lang="ts">
import { watch } from "vue"
import { $, $$ } from "vue/macros"
import { useRoute } from "vue-router"

import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"

const route = useRoute()
const { currentUser } = $(useCurrentUser())
const { loadInstanceInfo } = useInstanceInfo()

loadInstanceInfo()

watch($$(currentUser), () => {
  const title = currentUser ? `@${currentUser.username}` : "Federated social network"
  document.title = `Mitra | ${title}`
}, { immediate: true })
</script>

<style lang="scss">
@import "styles/reset";
@import "styles/layout";
@import "styles/mixins";
@import "styles/theme";

:root {
  --background-color: #{$background-color};
  --background-image: #{$background-image};
  --text-color: #{$text-color};
  --secondary-text-color: #{$secondary-text-color};
  --secondary-text-hover-color: #{$secondary-text-hover-color};
  --secondary-text-disabled-color: #{$secondary-text-disabled-color};
  --link-color: #{$link-color};
  --link-hover-color: #{$link-hover-color};
  --btn-background-color: #{$btn-background-color};
  --btn-background-hover-color: #{$btn-background-hover-color};
  --btn-text-color: #{$btn-text-color};
  --btn-text-hover-color: #{$btn-text-hover-color};
  --btn-secondary-background-color: #{$btn-secondary-background-color};
  --btn-secondary-text-color: #{$btn-secondary-text-color};
  --btn-disabled-background-color: #{$btn-disabled-background-color};
  --btn-disabled-text-color: #{$btn-disabled-text-color};
  --block-background-color: #{$block-background-color};
  --block-link-color: #{$block-link-color};
  --block-link-hover-color: #{$block-link-hover-color};
  --block-external-link-color: #{$block-external-link-color};
  --block-external-link-hover-color: #{$block-external-link-hover-color};
  --separator-color: #{$separator-color};
  --widget-background-color: #{$widget-background-color};
  --highlight-color: #{$highlight-color};
  --loader-color: #{$loader-color};
  --shadow-color: #{$shadow-color};
}

[data-theme="dark"] {
  --background-color: #{$dark-background-color};
  --background-image: #{$dark-background-image};
  --text-color: #{$dark-text-color};
  --secondary-text-color: #{$dark-secondary-text-color};
  --secondary-text-hover-color: #{$dark-secondary-text-hover-color};
  --secondary-text-disabled-color: #{$dark-secondary-text-disabled-color};
  --link-color: #{$dark-link-color};
  --link-hover-color: #{$dark-link-hover-color};
  --btn-background-color: #{$dark-btn-background-color};
  --btn-background-hover-color: #{$dark-btn-background-hover-color};
  --btn-text-color: #{$dark-btn-text-color};
  --btn-text-hover-color: #{$dark-btn-text-hover-color};
  --btn-secondary-background-color: #{$dark-btn-secondary-background-color};
  --btn-secondary-text-color: #{$dark-btn-secondary-text-color};
  --btn-disabled-background-color: #{$dark-btn-disabled-background-color};
  --btn-disabled-text-color: #{$dark-btn-disabled-text-color};
  --block-background-color: #{$dark-block-background-color};
  --block-link-color: #{$dark-block-link-color};
  --block-link-hover-color: #{$dark-block-link-hover-color};
  --block-external-link-color: #{$dark-block-external-link-color};
  --block-external-link-hover-color: #{$dark-block-external-link-hover-color};
  --separator-color: #{$dark-separator-color};
  --widget-background-color: #{$dark-widget-background-color};
  --highlight-color: #{$dark-highlight-color};
  --loader-color: #{$dark-loader-color};
  --shadow-color: #{$dark-shadow-color};
}

html {
  @include main-background;

  color: var(--text-color);
  font-family: $text-font;
  font-size: $text-font-size;
  min-height: 100%;

  &:has(.lightbox) {
    overflow: hidden;
  }
}

a {
  color: var(--link-color);
  cursor: pointer;
  text-decoration: none;

  &:hover {
    color: var(--link-hover-color);
  }
}

h1,
h2,
h3,
h4,
h5,
h6 {
  font-weight: bold;
  margin: 0 0 $block-outer-padding;
}

h1 {
  font-size: 2.2em;
}

h2 {
  font-size: 1.6em;
}

h3 {
  font-size: 1.4em;
}

h4 {
  font-size: 1.2em;
}

h5 {
  font-size: 1.1em;
}

h6 {
  font-size: 1em;
}

.static-text {
  line-height: 2;

  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    margin: 0;
  }

  p a {
    text-decoration: underline;
    text-decoration-skip-ink: none;

    &:hover {
      text-decoration: none;
    }
  }

  ol,
  ul {
    padding-left: 1.5em;
  }

  ul {
    list-style-type: disc;
  }
}

input,
textarea,
select {
  font-family: $text-font;
  font-size: $text-font-size;
}

input[type="number"],
input[type="password"],
input[type="search"],
input[type="text"],
textarea {
  background-color: var(--block-background-color);
  border: 1px solid var(--block-background-color);
  box-shadow: none;
  box-sizing: border-box;
  color: var(--text-color);
  margin: 0;
  padding: $input-padding;
  width: 100%;

  &:focus {
    outline: none;
  }

  &::placeholder {
    color: var(--secondary-text-color);
    opacity: 1;
  }
}

input[type="file"] {
  background-color: transparent;
  border: none;
  padding: 2px 1px;
}

input[type="checkbox"] {
  margin-right: $input-padding;
}

textarea {
  resize: vertical;
}

button {
  background-color: transparent;
  border: none;
  color: var(--link-color);
  cursor: pointer;
  font-family: $text-font;
  font-size: $text-font-size;
  padding: 0;
  text-align: left;

  &:hover {
    color: var(--link-hover-color);
  }

  &[disabled] {
    cursor: initial;
  }
}

.btn {
  background-color: var(--btn-background-color);
  border: none;
  border-radius: $btn-border-radius;
  box-shadow: $btn-shadow-size var(--shadow-color);
  color: var(--btn-text-color);
  cursor: pointer;
  display: inline-block;
  font-size: $text-font-size;
  font-weight: bold;
  padding: $input-padding 30px;
  text-align: center;
  white-space: nowrap;

  &:hover {
    background-color: var(--btn-background-hover-color);
    color: var(--btn-text-hover-color);
  }

  &[disabled] {
    background-color: var(--btn-disabled-background-color) !important;
    box-shadow: none;
    color: var(--btn-disabled-text-color) !important;
  }
}

.btn.secondary {
  background-color: var(--btn-secondary-background-color);
  color: var(--btn-secondary-text-color);

  &:hover {
    background-color: var(--btn-background-hover-color);
    color: var(--btn-text-hover-color);
  }
}

menu {
  list-style: none;
  margin: 0;
}

.wide {
  /* Reserve space for floating avatar */
  padding: $body-padding $content-gap * 1.5;
}

@media screen and (max-width: $screen-breakpoint-medium) {
  .wide {
    padding: $body-padding;
  }
}
</style>
