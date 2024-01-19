<script>
  import '../app.css';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
  import Prism from 'prismjs';
  import 'prismjs/components/prism-toml';
  import 'prismjs/components/prism-rust';
  import 'prismjs/components/prism-python';
  import 'prismjs/themes/prism-tomorrow.css';
  import { afterUpdate } from 'svelte';

  let message = '';
  let fileContent = '';
  let fileType = '';
  let fileName = '';
  let fileColors = '';

  function clearOtherMessage() {
    fileContent = '';
    fileType = '';
    fileName = '';
  }

  async function getMessage() {
    invoke('my_custom_command')
      .then((inMessage) => message = inMessage)
      .catch((err) => message = err);
  }

  async function showFileDialog() {
    clearOtherMessage();
    invoke('get_file_path')
      .catch((err) => fileContent = err);
  }

  listen('file-path', event => {
    fileContent = event.payload;
  });

  listen('file-type', event => {
    clearOtherMessage();
    fileType = event.payload;
    if (fileType === 'rs') {
      fileColors = 'language-rust';
    } else if (fileType === 'py') {
      fileColors = 'language-python';
    } else if (fileType === 'toml') {
      fileColors = 'language-toml';
    } else {
      fileColors = '';
    }
  });

  listen('file-name', event => {
    fileName = event.payload;
  });

  afterUpdate(() => {
    Prism.highlightAll();
  });

</script>

<button on:click={showFileDialog} class='btn bg-neutral hover:bg-neutral-focus'>File Dialog</button>
<button on:click={clearOtherMessage} class='btn bg-accent hover:bg-accent-focus'>Clear</button>
{#if fileContent !== ""}
  <h2 class='pl-4'>{fileName}</h2>
  <h2 class='pl-4'>{fileType}</h2>
  <pre><code class={fileColors}>{fileContent}</code></pre>
{/if}