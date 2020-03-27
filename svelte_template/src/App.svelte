<script>
	import wasm from './../../rust_wasm/Cargo.toml';

	const get_add = async ()=> {
		const exports = await wasm();
		return exports.add;
	};

	export let name;
</script>

<main>
	<h1>Hello {name}!</h1>
	<!-- <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial Nr {add(1 , 2)}</a> to learn how to build Svelte apps.</p> -->
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial Nr</a> to learn how to build Svelte apps.</p>

	{#await get_add()}
	awaiting add...
	{:then add}
		adding 1 and 2 results in {add(1,2)}
	{:catch error}
		some error is {error}
	{/await}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
