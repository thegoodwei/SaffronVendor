<?php
/*
Plugin Name: Ethereum Login
*/

// Enqueue the necessary scripts and dependencies
function enqueue_ethereum_login_scripts() {
    // Enqueue the Ethereum.js library
    wp_enqueue_script( 'ethereum-js', 'https://cdn.jsdelivr.net/npm/ethereum.js@latest', array(), '1.0.0', true );

    // Enqueue the plugin's JavaScript file
    wp_enqueue_script( 'ethereum-login', plugin_dir_url( __FILE__ ) . 'ethereum-login.js', array(), '1.0.0', true );

    // Inject the plugin's JavaScript object into the page
    wp_add_inline_script( 'ethereum-login', '
        const ethereumLogin = new EthereumLogin({
            web3: web3,
            nonce: "' . wp_create_nonce( 'ethereum-login-nonce' ) . '"
        });
    ');
}
add_action( 'wp_enqueue_scripts', 'enqueue_ethereum_login_scripts' );

// Add a custom login form to the login page
function ethereum_login_form() {
    $output = '<form id="ethereum-login-form" method="post">';
    $output .= '<label for="ethereum-address">Ethereum Address:</label><br>';
    $output .= '<input type="text" id="ethereum-address" name="ethereum-address"><br>';
    $output .= '<label for="ethereum-signature">Ethereum Signature:</label><br>';
    $output .= '<input type="text" id="ethereum-signature" name="ethereum-signature"><br><br>';
    $output .= '<input type="submit" value="Sign In">';
    $output .= '</form>';

    return $output;
}
add_filter( 'login_form_middle', 'ethereum_login_form' );

// Handle the login request
function ethereum_login() {
    // Check if the form was submitted
    if ( isset( $_POST['ethereum-address'] ) && isset( $_POST['ethereum-signature'] ) ) {
        // Verify the nonce
        if ( ! wp_verify_nonce( $_POST['ethereum-login-nonce'], 'ethereum-login-nonce' ) ) {
            wp_die( 'Invalid nonce' );
        }

        // Get the Ethereum address and signature from the form
        $address = sanitize_text_field( $_POST['ethereum-address'] );
        $signature = sanitize_text_field( $_POST['ethereum-signature'] );

        // Use the ethereumLogin object to verify the signature
        $result = ethereumLogin->verifySignature( $address, $signature );
        if ( ! $result ) {
            wp_die( 'Invalid signature' );
        }

        // // If the signature is valid, check if the user already exists in the WordPress database
        $user = get_user_by( 'login', $address );
        if ( ! $user ) {
            // If the user doesn't exist, create a new user
            $password = wp_generate_password();
            $user_id = wp_create_user( $address, $password );
        } else {
            $user_id = $user->ID;
        }
        wp_set_current_user( $user_id, $address );
        wp_set_auth_cookie( $user_id );
        wp_redirect( home_url() );
        exit;
    }
}
add_action( 'login_form_login', 'ethereum_login' );

// Add a custom logout link to the WordPress menu
function ethereum_logout_link( $items, $args ) {
    if ( is_user_logged_in() ) {
        $items .= '<li><a href="' . wp_logout_url( home_url() ) . '">Logout</a></li>';
    }
    return $items;
}
add_filter( 'wp_nav_menu_items', 'ethereum_logout_link', 10, 2 );

// Set up a named pipe for communication with Rust
$pipe_name = '/path/to/web3_user_pipe';
if ( ! file_exists( $pipe_name ) ) {
    if ( ! posix_mkfifo( $pipe_name, 0666 ) ) {
        exit( "Error creating named pipe" );
    }
}

// Open the named pipe for writing
$pipe = fopen( $pipe_name, 'w' );
if ( ! $pipe ) {
    exit( "Error opening named pipe" );
}

// Check if a wallet is connected
if ( ethereumLogin->isWalletConnected() ) {
    // Write a boolean value to the named pipe
    if ( fwrite( $pipe, true ) === false ) {
        exit( "Error writing to named pipe" );
    }
} else {
    // Write a boolean value to the named pipe
    if ( fwrite( $pipe, false ) === false ) {
        exit( "Error writing to named pipe" );
    }
}

fclose( $pipe );

