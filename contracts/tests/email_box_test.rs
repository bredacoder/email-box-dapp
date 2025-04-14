use contracts::email_box_proxy;
use multiversx_sc_scenario::imports::*;

const CODE_PATH: MxscPath = MxscPath::new("output/contracts.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts");
    blockchain.register_contract(CODE_PATH, contracts::ContractBuilder);
    blockchain
}

const OWNER: TestAddress = TestAddress::new("owner");
const USER2: TestAddress = TestAddress::new("user2");
const EMAIL_BOX_ADDRESS: TestSCAddress = TestSCAddress::new("email_box");

fn email_box_deploy() -> ScenarioWorld {
    let mut world = world();

    world.account(OWNER).nonce(0).balance(1000000);
    world.account(USER2).nonce(0).balance(1000000);

    let email_box_address = world
        .tx()
        .from(OWNER)
        .typed(email_box_proxy::EmailBoxProxy)
        .init()
        .code(CODE_PATH)
        .new_address(EMAIL_BOX_ADDRESS)
        .returns(ReturnsNewAddress)
        .run();

    assert_eq!(email_box_address, EMAIL_BOX_ADDRESS.to_address());

    world
}

#[test]
fn email_box_deploy_test() {
    let mut world = email_box_deploy();

    world
        .query()
        .to(EMAIL_BOX_ADDRESS)
        .typed(email_box_proxy::EmailBoxProxy)
        .get_max_preview_size()
        .returns(ExpectValue(100u32))
        .run();

    world
        .query()
        .to(EMAIL_BOX_ADDRESS)
        .typed(email_box_proxy::EmailBoxProxy)
        .get_max_content_size()
        .returns(ExpectValue(5242880u32))
        .run();
}

#[test]
fn email_box_send_email_test() {
    let mut world = email_box_deploy();

    world
        .tx()
        .from(OWNER)
        .to(EMAIL_BOX_ADDRESS)
        .typed(email_box_proxy::EmailBoxProxy)
        .send_email(
            USER2,
            ManagedBuffer::new_from_bytes(b"Hello, World!"),
            ManagedBuffer::new_from_bytes(b"Hello, World!"),
            ManagedBuffer::new_from_bytes(b"Hello, World!"),
        )
        .run();
}
