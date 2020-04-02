<script>
    export let enc; // the wasm function
	export let dec; // the wasm function

    const key = "this is not a good passphrase";
</script>

<main>
	<p> {enc("veryprivate", key)} </p>
</main>
