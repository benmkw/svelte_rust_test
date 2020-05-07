<script>
  import Microscheme from "./Microscheme.svelte";
  import AgeTool from "./AgeTool.svelte";
  import Signify from "./Signify.svelte";

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
  export let interpret; // the wasm function
  export let sign; // the wasm function
  export let verify; // the wasm function

  // drawer stuff
  let myDrawer;
  let activeDrawer = "Microscheme";
  let drawerOpen = false;
  function setActiveDrawer(value) {
    activeDrawer = value;
    drawerOpen = false;
  }
</script>

<!-- Tutorial https://svelte.dev/tutorial/styling -->
<!-- https://sveltematerialui.com/demo/textfield -->

<div>
  <div class="drawer-container">
    <Drawer variant="modal" bind:this={myDrawer} bind:open={drawerOpen}>
      <Header>
        <Title>Rust wasm Tools</Title>
        <Subtitle>Select the Rust tool you want to use</Subtitle>
      </Header>
      <Content>
        <List>
          <Item
            href="javascript:void(0)"
            on:click={() => setActiveDrawer('Microscheme')}
            activated={activeDrawer === 'Microscheme'}>
            <Text>Microscheme</Text>
          </Item>
          <Item
            href="javascript:void(0)"
            on:click={() => setActiveDrawer('Age Tool')}
            activated={activeDrawer === 'Age Tool'}>
            <Text>Age Tool</Text>
          </Item>
          <Item
            href="javascript:void(0)"
            on:click={() => setActiveDrawer('Signify')}
            activated={activeDrawer === 'Signify'}>
            <Text>Signify</Text>
          </Item>
        </List>
      </Content>
    </Drawer>

    <Scrim />
    <AppContent class="app-content">
      <main class="main-content">
        <Button on:click={() => (drawerOpen = !drawerOpen)}>
          <Label>Select Wasm Tool</Label>
        </Button>
        <br />
        <pre class="status">Active: {activeDrawer}</pre>
        <!-- <div style="height: 700px;">&nbsp;</div> -->
        {#if activeDrawer == 'Microscheme'}
          <Microscheme {interpret} />
        {:else if activeDrawer == 'Age Tool'}
          <AgeTool {enc} {dec} />
        {:else if activeDrawer == 'Signify'}
          <Signify {sign} {verify}/>
        {/if}

      </main>
    </AppContent>
  </div>
</div>
