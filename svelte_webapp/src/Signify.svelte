<script>
  import Textfield, { Input, Textarea } from "@smui/textfield";
  import Drawer, {
    AppContent,
    Content,
    Header,
    Title,
    Subtitle,
    Scrim
  } from "@smui/drawer";
  import Button, { Label } from "@smui/button";
  import List, { Item, Text, Graphic, Separator, Subheader } from "@smui/list";

  export let sign; // the wasm function
  export let verify; // the wasm function

  let input = "Hello world";
  let output = "";

  const signer = () => {
    output = sign(input);
  };

    const verifyer = () => {
    output = verify(input);
  };
</script>

<style>
  /* https://material.io/develop/web/components/input-controls/text-field/ */
  .container :global(.mdc-text-field) {
    height: 300px;
  }

  .container {
    background-color: whitesmoke;
  }
</style>

<div class="container">
  <Textfield
    fullwidth
    textarea
    height="100px"
    bind:value={input}
    label="Please input your text" />
</div>

<Button on:click={signer}>Please sign my input</Button>
<Button on:click={verifyer}>Please sign my input</Button>

<div>{output !== '' ? 'Output is  ' + output : ''}</div>
