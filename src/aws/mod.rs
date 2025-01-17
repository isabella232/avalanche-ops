use std::io;

use aws_config::{self, meta::region::RegionProviderChain};
use aws_sdk_ec2::Region;
use aws_types::SdkConfig as AwsSdkConfig;
use log::info;
use serde::{Deserialize, Serialize};

pub mod cloudformation;
pub mod cloudwatch;
pub mod ec2;
pub mod envelope;
pub mod kms;
pub mod s3;
pub mod sts;

/// Loads an AWS config from default environments.
pub async fn load_config(reg: Option<String>) -> io::Result<AwsSdkConfig> {
    info!("loading AWS configuration for region {:?}", reg);
    let regp = RegionProviderChain::first_try(reg.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    let shared_config = aws_config::from_env().region(regp).load().await;
    Ok(shared_config)
}

/// Represents the current AWS resource status.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Resources {
    /// AWS STS caller loaded from its local environment.
    /// READ ONLY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<sts::Identity>,

    /// AWS region to create resources.
    /// MUST BE NON-EMPTY.
    #[serde(default)]
    pub region: String,

    /// Name of the bucket to store (or download from)
    /// the configuration and resources (e.g., S3).
    /// If not exists, it creates automatically.
    /// If exists, it skips creation and uses the existing one.
    /// MUST BE NON-EMPTY.
    #[serde(default)]
    pub s3_bucket: String,

    /// Region for s3 where database backup resides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_backup_s3_region: Option<String>,
    /// Bucket to download backups from.
    /// Non-empty to download the database for bootstrapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_backup_s3_bucket: Option<String>,
    /// Non-empty to download the database for bootstrapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_backup_s3_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_system_logs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_system_metrics: Option<bool>,

    /// AWS region to create resources.
    /// NON-EMPTY TO ENABLE HTTPS over NLB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlb_acm_certificate_arn: Option<String>,

    /// KMS CMK ID to encrypt resources.
    /// None if not created yet.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_cmk_id: Option<String>,
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_cmk_arn: Option<String>,

    /// EC2 key pair name for SSH access to EC2 instances.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_name: Option<String>,
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_path: Option<String>,

    /// CloudFormation stack name for EC2 instance role.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_ec2_instance_role: Option<String>,
    /// Instance profile ARN from "cloudformation_ec2_instance_role".
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_ec2_instance_profile_arn: Option<String>,

    /// CloudFormation stack name for VPC.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_vpc: Option<String>,
    /// VPC ID from "cloudformation_vpc".
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_vpc_id: Option<String>,
    /// Security group ID from "cloudformation_vpc".
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_vpc_security_group_id: Option<String>,
    /// Public subnet IDs from "cloudformation_vpc".
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_vpc_public_subnet_ids: Option<Vec<String>>,

    /// CloudFormation stack name of Auto Scaling Group (ASG)
    /// for anchor nodes.
    /// None if mainnet.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_asg_anchor_nodes: Option<String>,
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_asg_anchor_nodes_logical_id: Option<String>,

    /// CloudFormation stack name of Auto Scaling Group (ASG)
    /// for non-anchor nodes.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_asg_non_anchor_nodes: Option<String>,
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_asg_non_anchor_nodes_logical_id: Option<String>,

    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_asg_nlb_arn: Option<String>,
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_asg_nlb_target_group_arn: Option<String>,
    /// Only updated after creation.
    /// READ ONLY -- DO NOT SET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation_asg_nlb_dns_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_avalanche_metrics_namespace: Option<String>,
}

impl Default for Resources {
    fn default() -> Self {
        Self::default()
    }
}

impl Resources {
    pub fn default() -> Self {
        Self {
            identity: None,
            region: String::from("us-west-2"),

            s3_bucket: String::from(""),

            db_backup_s3_region: None,
            db_backup_s3_bucket: None,
            db_backup_s3_key: None,

            instance_system_logs: Some(true),
            instance_system_metrics: Some(true),

            nlb_acm_certificate_arn: None,

            kms_cmk_id: None,
            kms_cmk_arn: None,

            ec2_key_name: None,
            ec2_key_path: None,

            cloudformation_ec2_instance_role: None,
            cloudformation_ec2_instance_profile_arn: None,

            cloudformation_vpc: None,
            cloudformation_vpc_id: None,
            cloudformation_vpc_security_group_id: None,
            cloudformation_vpc_public_subnet_ids: None,

            cloudformation_asg_anchor_nodes: None,
            cloudformation_asg_anchor_nodes_logical_id: None,

            cloudformation_asg_non_anchor_nodes: None,
            cloudformation_asg_non_anchor_nodes_logical_id: None,

            cloudformation_asg_nlb_arn: None,
            cloudformation_asg_nlb_target_group_arn: None,
            cloudformation_asg_nlb_dns_name: None,

            cloudwatch_avalanche_metrics_namespace: None,
        }
    }
}
