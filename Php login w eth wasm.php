<?php
/*
Plugin Name: Ethereum Login
*/

function enqueue_ethereum_login_scripts() {
    // Enqueue the WASM module as a JavaScript file
    wp_enqueue_script( 'ethereum-login', plugin_dir_url( __FILE__ ) . 'ethereum-login.js', array(), '1.0.0', true );

    // Inject the WASM module into the page, passing in the necessary dependencies as arguments
    wp_add_inline_script( 'ethereum-login', '
        const ethereumLogin = new EthereumLogin({
            web3: web3,
            nonce: "' . wp_create_nonce( 'ethereum-login-nonce' ) . '"
        });
    ');
}
add_action( 'wp_enqueue_scripts', 'enqueue_ethereum_login_scripts' );
