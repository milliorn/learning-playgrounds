<script>
	// Wrap event listener code inside onMount
	import { onMount } from 'svelte';

	onMount(() => {
		// Add event listeners for keyboard input
		window.addEventListener('keydown', handleKeyPress);
		window.addEventListener('keyup', handleKeyUp);
	});

	let keyHeld = false; // To track if a key is held down

	// Handle keyboard input
	/**
	 * 	 * @param {{ key: any; }} event
	 */
	function handleKeyPress(event) {
		if (!keyHeld) {
			keyHeld = true;
			const key = event.key;

			if (key === 'Enter') {
				// Handle Enter key as equals
				equals();
			} else if (!isNaN(key) || key === '.') {
				// Handle numeric keys or decimal point
				select(key)();
			} else if (key === '+/-') {
				// Handle plus/minus key
				toggleSign();
			} else if (key === '+' || key === '-' || key === '*' || key === '/') {
				// Handle operator keys
				operation(key);
			} else if (key === 'Escape') {
				// Handle Escape key as clear
				clear();
			}
		}
	}

	// Handle key release
	function handleKeyUp() {
		keyHeld = false;
	}

	let display_number = '0';

	const select = (/** @type {string | number} */ num) => () =>
		display_number === '0' ? (display_number = num.toString()) : (display_number += num);

	const clear = () => (display_number = '0');

	/**
	 * @type {number | undefined}
	 */
	let operand;
	/**
	 * @type {number | undefined}
	 */
	let result;

	function equals() {
		// result is here to resolve a null issue with type checking.
		result = 0;
		const displayNumberAsNumber = Number(display_number);

		if (!isNaN(displayNumberAsNumber) && operand !== undefined) {
			switch (operator) {
				case '+':
					result = operand + displayNumberAsNumber;
					break;
				case '-':
					result = operand - displayNumberAsNumber;
					break;
				case '*':
					result = operand * displayNumberAsNumber;
					break;
				case '/':
					result = operand / displayNumberAsNumber;
					break;
				default:
					break; // Handle any other operator or no operator
			}
			display_number = result.toString();
		}
	}

	/**
	 * @type {string | undefined}
	 */
	let operator;
	let operators = ['+', '/', '*', '-'];

	/**
	 * @param {string} sign
	 */
	function operation(sign) {
		operand = Number(display_number);
		operator = sign;
		display_number = '';
	}

	function toggleSign() {
		if (display_number !== '0') {
			display_number = display_number.startsWith('-')
				? display_number.slice(1)
				: '-' + display_number;
		}
	}
</script>

<div class="calculator">
	<div class="display">
		{display_number.length <= 10 ? display_number : display_number.substring(0, 10)}
	</div>
	<div class="buttons">
		<button on:click={select(7)}>7</button>
		<button on:click={select(8)}>8</button>
		<button on:click={select(9)}>9</button>
		<button on:click={select(4)}>4</button>
		<button on:click={select(5)}>5</button>
		<button on:click={select(6)}>6</button>
		<button on:click={select(1)}>1</button>
		<button on:click={select(2)}>2</button>
		<button on:click={select(3)}>3</button>
		<button on:click={select(0)}>0</button>
		<button on:click={select('.')}>.</button>
		<button on:click={toggleSign}>+/-</button>
		<button on:click={() => operation(operators[0])} class="operator">+</button>
		<button on:click={() => operation(operators[1])} class="operator">/</button>
		<button on:click={() => operation(operators[2])} class="operator">*</button>
		<button on:click={() => operation(operators[3])} class="operator">-</button>
		<button on:click={equals} class="equals">=</button>
		<button on:click={clear} class="clear">C</button>
	</div>
</div>

<style>
	button {
		border-radius: 0.25rem; /* 4px */
		border: none;
		cursor: pointer;
		font-family: Arial, sans-serif;
		font-size: 1.125rem; /* 18px */
		line-height: 1.75rem; /* 28px */
		padding: 0.625rem; /* 10px */
		text-align: center;
	}

	button:hover {
		background: #e5e7eb;
	}

	.buttons {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 0.25rem; /* 4px */
	}

	.calculator {
		background-color: #000;
		border-radius: 0.375rem; /* 6px */
		border: 1px solid rgb(214 211 209);
		box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
		padding: 0.625rem; /* 10px */
		width: 91.666667%;

		/* Center the calculator vertically and horizontally */
		align-items: center;
		display: flexbox;
		flex-direction: column;
		font-family: Arial, sans-serif;
		justify-content: center;
		left: 50%;
		position: absolute;
		top: 50%;
		transform: translate(-50%, -50%);
	}

	.clear {
		background: #dc2626;
		border: 0px solid #000;
	}

	.clear:hover {
		background: #fecaca;
	}

	.display {
		border: 1px solid rgb(214 211 209);
		color: white;
		font-size: 3rem; /* 48px */
		line-height: 2.5rem; /* 40px */
		margin-bottom: 0.625rem; /* 10px */
		min-height: 2.5rem; /* 40px */
		padding: 0.625rem; /* 10px */
		text-align: right;
	}

	.equals {
		background: #2563eb;
		border: 0px solid #000;
	}

	.equals:hover {
		background: #bae6fd;
	}

	.operator {
		background: #f59e0b;
		border: 0px solid #a16207;
	}

	.operator:hover {
		background: #facc15;
	}

	@media (min-width: 768px) {
		button {
			font-size: 3rem; /* 48px */
			line-height: 1;
		}

		.buttons {
			gap: 0.875rem; /* 14px */
		}

		.display {
			font-size: 6rem; /* 128px */
			line-height: 1;
			min-height: 6rem; /* 80px */
		}
	}

	@media (min-width: 1024px) {
		button {
			font-size: 6rem; /* 96px */
			line-height: 1;
		}

		.buttons {
			gap: 1rem; /* 16px */
		}

		.display {
			font-size: 7rem; /* 96px */
			min-height: 7rem; /* 80px */
		}
	}
</style>
