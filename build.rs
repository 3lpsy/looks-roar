use ethers::contract::Abigen;
use std::error::Error;
use std::fs;
const ABIS_DIR: &str = "./src/contract/abis/";
const CONTRACT_DIR: &str = "./src/contract/";
const ERC165: &str = r#"[
    function supportsInterface(bytes4 interfaceId) external view returns (bool)
]"#;
const ERC721: &str = r#"[
    function balanceOf(address _owner) external view returns (uint256)
    function ownerOf(uint256 _tokenId) external view returns (address)
    function safeTransferFrom(address _from, address _to, uint256 _tokenId, bytes data) external payable
    function safeTransferFrom(address _from, address _to, uint256 _tokenId) external payable;
    function transferFrom(address _from, address _to, uint256 _tokenId) external payable
    function approve(address _approved, uint256 _tokenId) external payable
    function setApprovalForAll(address _operator, bool _approved) external
    function getApproved(uint256 _tokenId) external view returns (address)
    function isApprovedForAll(address _owner, address _operator) external view returns (bool)
    function supportsInterface(bytes4 interfaceId) external view returns (bool)
]"#;
const ERC721_METADATA: &str = r#"[
    function name() external view returns (string _name)
    function symbol() external view returns (string _symbol)
    function tokenURI(uint256 _tokenId) external view returns (string)
]"#;
const ERC721_ENUMERABLE: &str = r#"[
    function totalSupply() external view returns (uint256)
    function tokenByIndex(uint256 _index) external view returns (uint256)
    function tokenOfOwnerByIndex(address _owner, uint256 _index) external view returns (uint256)
]"#;
const ERC1155: &str = r#"[
    function safeTransferFrom(address _from, address _to, uint256 _id, uint256 _value, bytes calldata _data) external
    function safeBatchTransferFrom(address _from, address _to, uint256[] calldata _ids, uint256[] calldata _values, bytes calldata _data) external
    function balanceOf(address _owner) external view returns (uint256)
    function balanceOfBatch(address[] calldata _owners, uint256[] calldata _ids) external view returns (uint256[] memory)
    function setApprovalForAll(address _operator, bool _approved) external
    function isApprovedForAll(address _owner, address _operator) external view returns (bool)
    event TransferSingle(address indexed _operator, address indexed _from, address indexed _to, uint256 _id, uint256 _value)
    event TransferBatch(address indexed _operator, address indexed _from, address indexed _to, uint256[] _ids, uint256[] _values)
    event URI(string _value, uint256 indexed _id)
    event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved)
]"#;
const ERC1155_METADATA_URI: &str = r#"[
    function uri(uint256 _id) external view returns (string memory)
]"#;
fn main() -> Result<(), Box<dyn Error>> {
    clean().unwrap();
    let mut abismod = String::new();
    generate("erc165", "ERC165", ERC165, &mut abismod).unwrap();
    generate("erc721", "ERC721", ERC721, &mut abismod).unwrap();
    generate(
        "erc721_enumerable",
        "ERC721Enumerable",
        ERC721_ENUMERABLE,
        &mut abismod,
    )
    .unwrap();
    generate(
        "erc721_metadata",
        "ERC721Metadata",
        ERC721_METADATA,
        &mut abismod,
    )
    .unwrap();
    generate("erc1155", "ERC1155", ERC1155, &mut abismod).unwrap();
    generate(
        "erc1155_metadata_uri",
        "ERC1155MetadataUri",
        ERC1155_METADATA_URI,
        &mut abismod,
    )
    .unwrap();
    let abismod_path = format!("{}{}", CONTRACT_DIR, "abis.rs");
    fs::write(abismod_path, abismod).unwrap();
    Ok(())
}
fn clean() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(ABIS_DIR)? {
        let e = entry.unwrap();
        let p = String::from(e.path().to_string_lossy());
        if p.ends_with(".rs") {
            fs::remove_file(p)?
        }
    }
    Ok(())
}
fn generate(
    file_name: &str,
    contract_name: &str,
    abi: &str,
    abismod: &mut String,
) -> Result<(), Box<dyn Error>> {
    let path = format!("{}{}{}", ABIS_DIR, file_name, ".rs");
    Abigen::new(contract_name, abi)?
        .generate()?
        .write_to_file(path)?;

    let line1 = format!("pub mod {};\n", file_name);
    abismod.push_str(&line1);
    let line2 = format!("pub use {}::{};\n", file_name, contract_name);
    abismod.push_str(&line2);
    Ok(())
}
