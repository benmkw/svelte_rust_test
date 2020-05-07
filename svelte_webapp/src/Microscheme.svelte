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

  export let interpret; // the wasm function

  let input =
    "(define max (lambda (x) \n \
                      (if (= (cdr x) '()) \n \
                        (car x) \n \
                        (if (> (car x) (max (cdr x))) \n \
                            (car x) \n \
                            (max (cdr x)))))) \n \
 \n \
                    (max '(1 2 30 4))";
  let output = "";

  const run = () => {
    output = interpret(input);
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
    label="Please input your programm" />
</div>

<Button on:click={run}>Please Run my programm</Button>

<div>{output !== '' ? 'Output is  ' + output : ''}</div>
