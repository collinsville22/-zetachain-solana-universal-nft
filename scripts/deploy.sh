#!/bin/bash

# ZetaChain Solana Universal NFT Deployment Script
# This script handles the complete deployment process for devnet and mainnet

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROGRAM_NAME="universal_nft"
KEYPAIR_PATH="$HOME/.config/solana/id.json"
DEVNET_RPC="https://api.devnet.solana.com"
MAINNET_RPC="https://api.mainnet-beta.solana.com"

echo -e "${BLUE}🚀 ZetaChain Solana Universal NFT Deployment Script${NC}"
echo "=================================================="

# Function to check prerequisites
check_prerequisites() {
    echo -e "${YELLOW}📋 Checking prerequisites...${NC}"
    
    # Check if Solana CLI is installed
    if ! command -v solana &> /dev/null; then
        echo -e "${RED}❌ Solana CLI is not installed. Please install it first.${NC}"
        exit 1
    fi
    
    # Check if Anchor is installed
    if ! command -v anchor &> /dev/null; then
        echo -e "${RED}❌ Anchor CLI is not installed. Please install it first.${NC}"
        exit 1
    fi
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust is not installed. Please install it first.${NC}"
        exit 1
    fi
    
    # Check if keypair exists
    if [ ! -f "$KEYPAIR_PATH" ]; then
        echo -e "${RED}❌ Solana keypair not found at $KEYPAIR_PATH${NC}"
        echo "Please generate a keypair with: solana-keygen new"
        exit 1
    fi
    
    echo -e "${GREEN}✅ All prerequisites met${NC}"
}

# Function to get user confirmation
confirm() {
    read -p "$1 (y/N): " -n 1 -r
    echo
    [[ $REPLY =~ ^[Yy]$ ]]
}

# Function to build the program
build_program() {
    echo -e "${YELLOW}🔨 Building the program...${NC}"
    
    anchor build
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Program built successfully${NC}"
    else
        echo -e "${RED}❌ Program build failed${NC}"
        exit 1
    fi
}

# Function to run tests
run_tests() {
    echo -e "${YELLOW}🧪 Running tests...${NC}"
    
    anchor test --skip-local-validator
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ All tests passed${NC}"
    else
        echo -e "${RED}❌ Tests failed${NC}"
        if ! confirm "Continue with deployment despite test failures?"; then
            exit 1
        fi
    fi
}

# Function to check balance
check_balance() {
    local cluster=$1
    echo -e "${YELLOW}💰 Checking balance on $cluster...${NC}"
    
    solana config set --url $2
    local balance=$(solana balance --keypair $KEYPAIR_PATH | cut -d' ' -f1)
    local balance_float=$(echo "$balance" | awk '{print $1+0}')
    
    echo "Current balance: $balance SOL"
    
    if (( $(echo "$balance_float < 1" | bc -l) )); then
        echo -e "${RED}⚠️  Warning: Low balance ($balance SOL). You may need more SOL for deployment.${NC}"
        if [ "$cluster" = "devnet" ]; then
            echo "You can request devnet SOL from: https://faucet.solana.com/"
        fi
        if ! confirm "Continue with deployment?"; then
            exit 1
        fi
    else
        echo -e "${GREEN}✅ Sufficient balance for deployment${NC}"
    fi
}

# Function to deploy to network
deploy_to_network() {
    local network=$1
    local rpc_url=$2
    
    echo -e "${YELLOW}🚀 Deploying to $network...${NC}"
    
    # Set Solana config to target network
    solana config set --url $rpc_url
    
    # Check balance
    check_balance $network $rpc_url
    
    # Deploy the program
    echo "Deploying program..."
    anchor deploy --provider.cluster $network
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Successfully deployed to $network${NC}"
        
        # Get program ID
        PROGRAM_ID=$(solana address -k target/deploy/${PROGRAM_NAME}-keypair.json)
        echo "Program ID: $PROGRAM_ID"
        
        # Verify deployment
        solana program show $PROGRAM_ID
        
        echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
        echo "Program ID: $PROGRAM_ID"
        echo "Network: $network"
        echo "RPC: $rpc_url"
        
    else
        echo -e "${RED}❌ Deployment failed${NC}"
        exit 1
    fi
}

# Function to initialize the program
initialize_program() {
    local network=$1
    echo -e "${YELLOW}⚙️  Initializing program on $network...${NC}"
    
    # This would run the initialization script
    # For now, we'll just provide instructions
    echo "To initialize the program, run:"
    echo "anchor run initialize --provider.cluster $network"
    echo ""
    echo "Make sure to:"
    echo "1. Set the correct gateway authority"
    echo "2. Configure TSS authority"
    echo "3. Verify all settings"
}

# Function to display post-deployment info
post_deployment_info() {
    local network=$1
    echo -e "${BLUE}📋 Post-Deployment Information${NC}"
    echo "================================="
    echo ""
    echo -e "${GREEN}✅ Deployment Summary:${NC}"
    echo "Network: $network"
    echo "Program ID: $(solana address -k target/deploy/${PROGRAM_NAME}-keypair.json)"
    echo "Deployer: $(solana address --keypair $KEYPAIR_PATH)"
    echo ""
    echo -e "${YELLOW}📋 Next Steps:${NC}"
    echo "1. Initialize the program with correct authorities"
    echo "2. Verify integration with ZetaChain gateway"
    echo "3. Run integration tests"
    echo "4. Update frontend/client configurations"
    echo ""
    echo -e "${YELLOW}🔗 Useful Links:${NC}"
    if [ "$network" = "devnet" ]; then
        echo "- Solana Explorer: https://explorer.solana.com/?cluster=devnet"
        echo "- RPC Endpoint: $DEVNET_RPC"
    else
        echo "- Solana Explorer: https://explorer.solana.com/"
        echo "- RPC Endpoint: $MAINNET_RPC"
    fi
    echo "- ZetaChain Docs: https://docs.zetachain.com/"
}

# Function to generate deployment report
generate_report() {
    local network=$1
    local report_file="deployment_report_${network}_$(date +%Y%m%d_%H%M%S).txt"
    
    echo "Generating deployment report: $report_file"
    
    cat > $report_file << EOF
ZetaChain Solana Universal NFT Deployment Report
==============================================

Deployment Date: $(date)
Network: $network
Program ID: $(solana address -k target/deploy/${PROGRAM_NAME}-keypair.json)
Deployer: $(solana address --keypair $KEYPAIR_PATH)
Solana CLI Version: $(solana --version)
Anchor Version: $(anchor --version)

Build Information:
- Rust Version: $(rustc --version)
- Program Size: $(ls -lh target/deploy/${PROGRAM_NAME}.so | awk '{print $5}')

Deployment Status: SUCCESS

Post-Deployment Checklist:
[ ] Program initialized with correct authorities
[ ] Gateway integration verified
[ ] Integration tests passed
[ ] Frontend configuration updated
[ ] Documentation updated with new Program ID

Notes:
- Remember to update Anchor.toml with the new Program ID
- Verify the program on Solana Explorer
- Test all cross-chain functionality before production use

EOF

    echo -e "${GREEN}✅ Deployment report generated: $report_file${NC}"
}

# Main deployment flow
main() {
    echo "Select deployment target:"
    echo "1) Devnet (recommended for testing)"
    echo "2) Mainnet (production deployment)"
    echo "3) Build and test only"
    read -p "Enter your choice (1-3): " choice
    
    case $choice in
        1)
            echo -e "${YELLOW}📍 Deploying to Devnet${NC}"
            check_prerequisites
            build_program
            run_tests
            deploy_to_network "devnet" $DEVNET_RPC
            initialize_program "devnet"
            post_deployment_info "devnet"
            generate_report "devnet"
            ;;
        2)
            echo -e "${RED}⚠️  MAINNET DEPLOYMENT${NC}"
            echo "This will deploy to Solana mainnet with real SOL!"
            if ! confirm "Are you absolutely sure you want to deploy to mainnet?"; then
                echo "Deployment cancelled."
                exit 0
            fi
            check_prerequisites
            build_program
            run_tests
            deploy_to_network "mainnet" $MAINNET_RPC
            initialize_program "mainnet"
            post_deployment_info "mainnet"
            generate_report "mainnet"
            ;;
        3)
            echo -e "${YELLOW}🔨 Build and test only${NC}"
            check_prerequisites
            build_program
            run_tests
            echo -e "${GREEN}✅ Build and test completed${NC}"
            ;;
        *)
            echo -e "${RED}❌ Invalid choice${NC}"
            exit 1
            ;;
    esac
}

# Handle script interruption
trap 'echo -e "\n${RED}❌ Deployment interrupted${NC}"; exit 1' INT

# Run main function
main "$@"