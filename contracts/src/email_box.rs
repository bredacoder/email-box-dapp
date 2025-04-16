#![no_std]

use multiversx_sc::imports::*;
#[allow(unused_imports)]
use multiversx_sc::{derive_imports::*, imports::*};

pub mod email_box_proxy;

// EmailSummary struct - Contains all the essential metadata about an email
// This structure is used to store email information without the full content
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct EmailSummary<M: ManagedTypeApi> {
    pub from: ManagedAddress<M>,     // Sender's address
    pub to: ManagedAddress<M>,       // Recipient's address
    pub subject: ManagedBuffer<M>,   // Email subject line
    pub preview: ManagedBuffer<M>,   // Short preview of the email content
    pub timestamp: u64,              // When the email was sent
    pub ipfs_hash: ManagedBuffer<M>, // IPFS hash pointing to the full email content
}

#[multiversx_sc::contract]
pub trait EmailBox {
    // Contract initialization - Sets default size limits
    #[init]
    fn init(&self) {
        self.max_preview_size().set(100u32); // Default preview size: 100 bytes
        self.max_content_size().set(5 * 1024 * 1024); // Default content size: 5MB
    }

    // Storage mappers - Define where and how data is stored on the blockchain

    // Stores received emails for each user
    #[storage_mapper("inbox")]
    fn inbox(&self, user: &ManagedAddress) -> VecMapper<EmailSummary<Self::Api>>;

    // Stores sent emails for each user
    #[storage_mapper("sent")]
    fn sent(&self, user: &ManagedAddress) -> VecMapper<EmailSummary<Self::Api>>;

    // Stores the maximum preview size (in bytes)
    #[storage_mapper("maxPreviewSize")]
    fn max_preview_size(&self) -> SingleValueMapper<u32>;

    // Stores the maximum content size (in bytes)
    #[storage_mapper("maxContentSize")]
    fn max_content_size(&self) -> SingleValueMapper<u32>;

    // Valid size ranges - Define acceptable min and max values for configuration

    // Preview size: 50 bytes to 500 KB
    fn valid_preview_range(&self) -> (u32, u32) {
        (50, 500 * 1024)
    }

    // Content size: 1 byte to 5MB
    fn valid_content_range(&self) -> (u32, u32) {
        (1, 5 * 1024 * 1024)
    }

    // Main endpoint for sending an email
    #[endpoint(sendEmail)]
    fn send_email(
        &self,
        to: ManagedAddress,          // Recipient address
        subject: ManagedBuffer,      // Email subject
        full_content: ManagedBuffer, // Full email content
        ipfs_hash: ManagedBuffer,    // IPFS hash for retrieving content
    ) {
        // Get the sender's address from the blockchain
        let from = self.blockchain().get_caller();
        // Get the current block timestamp
        let timestamp = self.blockchain().get_block_timestamp();

        // Check content size against limits
        let content_len = full_content.len();
        let max_preview_size = self.max_preview_size().get() as usize;
        let max_content_size = self.max_content_size().get() as usize;

        // Ensure the email content is not too large
        require!(content_len <= max_content_size, "Message too large!");

        // Generate preview: either truncate content or use full content if small enough
        let preview: ManagedBuffer<Self::Api> = if content_len > max_preview_size {
            full_content
                .copy_slice(0, max_preview_size)
                .expect("slice out of bounds")
        } else {
            full_content.clone()
        };

        // Create email summary with all metadata
        let summary = EmailSummary {
            from: from.clone(),
            to: to.clone(),
            subject,
            preview,
            timestamp,
            ipfs_hash,
        };

        // Store the email in recipient's inbox and sender's sent folder
        self.inbox(&to).push(&summary);
        self.sent(&from).push(&summary);

        // Emit event for email sent
        self.email_sent(summary);
    }

    // Configuration endpoints - Only the contract owner can call these

    // Change the maximum preview size
    #[only_owner]
    #[endpoint(setMaxPreviewSize)]
    fn set_max_preview_size(&self, size: u32) {
        let (min, max) = self.valid_preview_range();
        require!(
            size >= min && size <= max,
            "Preview size must be between 50 bytes and 500 KB"
        );
        self.max_preview_size().set(size);
    }

    // Change the maximum content size
    #[only_owner]
    #[endpoint(setMaxContentSize)]
    fn set_max_content_size(&self, size: u32) {
        let (min, max) = self.valid_content_range();
        require!(
            size >= min && size <= max,
            "Content size must be between 1 byte and 5MB"
        );
        self.max_content_size().set(size);
    }

    // View functions - Read-only access to contract data

    // Get the current maximum preview size
    #[view(getMaxPreviewSize)]
    fn get_max_preview_size(&self) -> u32 {
        self.max_preview_size().get()
    }

    // Get the current maximum content size
    #[view(getMaxContentSize)]
    fn get_max_content_size(&self) -> u32 {
        self.max_content_size().get()
    }

    // View inbox with pagination
    // Parameters:
    // - limit: maximum number of emails to return
    // - offset: number of emails to skip before starting to return
    #[view(getInbox)]
    fn get_inbox(&self, limit: u32, offset: u32) -> ManagedBuffer<Self::Api> {
        // Only allow users to access their own inbox
        let caller = self.blockchain().get_caller();
        let inbox = self.inbox(&caller);
        let total = inbox.len();

        // Calculate start and end indices with bounds checking
        let start = offset.min(total as u32) as usize;
        let end = ((offset + limit).min(total as u32)) as usize;

        // Iterate and collect only the desired slice of emails
        let selected_emails = inbox.iter().skip(start).take(end - start);

        // Serialize emails to a buffer for return
        let mut serialized_emails = ManagedBuffer::new();
        for email in selected_emails {
            email.top_encode(&mut serialized_emails).unwrap();
        }

        serialized_emails
    }

    // Event definition - Triggered when an email is sent
    #[event]
    fn email_sent(&self, email_summary: EmailSummary<Self::Api>);
}
