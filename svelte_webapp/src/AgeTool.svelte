<script>
  import "./AgeTool.scss";

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

  export let enc; // the wasm function
  export let dec; // the wasm function

  let key = "very secure default key";
  let input = "";
  let output = "";

  const encrypt = () => {
    // output = interpret(input, key);
    output = enc(input, key);
  };
  const decrypt = () => {
    output = dec(output, key);
  };

  let decrypted = "";
</script>

<style>
  .margins {
    margin: 60px 0 60px 60px;
  }
</style>

<div class="margins">
  <Textfield
    fullwidth
    textarea
    bind:value={key}
    label="Please input you very secure key" />
</div>

<div class="margins">
  <Textfield
    fullwidth
    textarea
    bind:value={input}
    label="Please input not encrypted text" />
</div>

<Button on:click={encrypt}>Please Encrypt my text</Button>

<div>{output}</div>

<div class="container">
  <Button on:click={decrypt}>Please decrypt my text</Button>
</div>

<div>{decrypted}</div>
