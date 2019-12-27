-- MySQL dump 10.13  Distrib 5.7.21, for Linux (x86_64)
--
-- Host: 127.0.0.1    Database: ApolloConfigDB
-- ------------------------------------------------------
-- Server version	5.7.28

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Current Database: `ApolloConfigDB`
--

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `ApolloConfigDB` /*!40100 DEFAULT CHARACTER SET utf8mb4 */;

USE `ApolloConfigDB`;

--
-- Table structure for table `App`
--

DROP TABLE IF EXISTS `App`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `App` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `AppId` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `Name` varchar(500) NOT NULL DEFAULT 'default' COMMENT '应用名',
  `OrgId` varchar(32) NOT NULL DEFAULT 'default' COMMENT '部门Id',
  `OrgName` varchar(64) NOT NULL DEFAULT 'default' COMMENT '部门名字',
  `OwnerName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ownerName',
  `OwnerEmail` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ownerEmail',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `AppId` (`AppId`(191)),
  KEY `DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_Name` (`Name`(191))
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COMMENT='应用表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `App`
--

LOCK TABLES `App` WRITE;
/*!40000 ALTER TABLE `App` DISABLE KEYS */;
INSERT INTO `App` VALUES (1,'SampleApp','Sample App','TEST1','样例部门1','apollo','apollo@acme.com','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54');
/*!40000 ALTER TABLE `App` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `AppNamespace`
--

DROP TABLE IF EXISTS `AppNamespace`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `AppNamespace` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `Name` varchar(32) NOT NULL DEFAULT '' COMMENT 'namespace名字，注意，需要全局唯一',
  `AppId` varchar(32) NOT NULL DEFAULT '' COMMENT 'app id',
  `Format` varchar(32) NOT NULL DEFAULT 'properties' COMMENT 'namespace的format类型',
  `IsPublic` bit(1) NOT NULL DEFAULT b'0' COMMENT 'namespace是否为公共',
  `Comment` varchar(64) NOT NULL DEFAULT '' COMMENT '注释',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT '' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_AppId` (`AppId`),
  KEY `Name_AppId` (`Name`,`AppId`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COMMENT='应用namespace定义';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `AppNamespace`
--

LOCK TABLES `AppNamespace` WRITE;
/*!40000 ALTER TABLE `AppNamespace` DISABLE KEYS */;
INSERT INTO `AppNamespace` VALUES (1,'application','SampleApp','properties','\0','default app namespace','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'application.xml','SampleApp','xml','\0','','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(3,'application.json','SampleApp','json','\0','','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(4,'application.yml','SampleApp','yml','\0','','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(5,'application.txt','SampleApp','txt','\0','','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12');
/*!40000 ALTER TABLE `AppNamespace` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Audit`
--

DROP TABLE IF EXISTS `Audit`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Audit` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `EntityName` varchar(50) NOT NULL DEFAULT 'default' COMMENT '表名',
  `EntityId` int(10) unsigned DEFAULT NULL COMMENT '记录ID',
  `OpName` varchar(50) NOT NULL DEFAULT 'default' COMMENT '操作类型',
  `Comment` varchar(500) DEFAULT NULL COMMENT '备注',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=25 DEFAULT CHARSET=utf8mb4 COMMENT='日志审计表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Audit`
--

LOCK TABLES `Audit` WRITE;
/*!40000 ALTER TABLE `Audit` DISABLE KEYS */;
INSERT INTO `Audit` VALUES (1,'Namespace',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:14',NULL,'2019-12-27 13:27:14'),(2,'AppNamespace',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:14',NULL,'2019-12-27 13:27:14'),(3,'Namespace',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:26',NULL,'2019-12-27 13:27:26'),(4,'AppNamespace',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:26',NULL,'2019-12-27 13:27:26'),(5,'Namespace',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:03',NULL,'2019-12-27 13:28:03'),(6,'AppNamespace',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:03',NULL,'2019-12-27 13:28:03'),(7,'Namespace',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:12',NULL,'2019-12-27 13:28:12'),(8,'AppNamespace',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:12',NULL,'2019-12-27 13:28:12'),(9,'Item',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:52',NULL,'2019-12-27 13:28:52'),(10,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:52',NULL,'2019-12-27 13:28:52'),(11,'Release',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:54',NULL,'2019-12-27 13:28:54'),(12,'ReleaseHistory',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:54',NULL,'2019-12-27 13:28:54'),(13,'Item',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:54',NULL,'2019-12-27 13:29:54'),(14,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:54',NULL,'2019-12-27 13:29:54'),(15,'Release',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:56',NULL,'2019-12-27 13:29:56'),(16,'ReleaseHistory',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:56',NULL,'2019-12-27 13:29:56'),(17,'Item',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:25',NULL,'2019-12-27 13:30:25'),(18,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:25',NULL,'2019-12-27 13:30:25'),(19,'Release',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:27',NULL,'2019-12-27 13:30:27'),(20,'ReleaseHistory',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:27',NULL,'2019-12-27 13:30:27'),(21,'Item',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:37',NULL,'2019-12-27 13:30:37'),(22,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:37',NULL,'2019-12-27 13:30:37'),(23,'Release',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:38',NULL,'2019-12-27 13:30:38'),(24,'ReleaseHistory',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:38',NULL,'2019-12-27 13:30:38');
/*!40000 ALTER TABLE `Audit` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Cluster`
--

DROP TABLE IF EXISTS `Cluster`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Cluster` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `Name` varchar(32) NOT NULL DEFAULT '' COMMENT '集群名字',
  `AppId` varchar(32) NOT NULL DEFAULT '' COMMENT 'App id',
  `ParentClusterId` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '父cluster',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT '' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_AppId_Name` (`AppId`,`Name`),
  KEY `IX_ParentClusterId` (`ParentClusterId`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COMMENT='集群';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Cluster`
--

LOCK TABLES `Cluster` WRITE;
/*!40000 ALTER TABLE `Cluster` DISABLE KEYS */;
INSERT INTO `Cluster` VALUES (1,'default','SampleApp',0,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54');
/*!40000 ALTER TABLE `Cluster` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Commit`
--

DROP TABLE IF EXISTS `Commit`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Commit` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `ChangeSets` longtext NOT NULL COMMENT '修改变更集',
  `AppId` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `ClusterName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ClusterName',
  `NamespaceName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'namespaceName',
  `Comment` varchar(500) DEFAULT NULL COMMENT '备注',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`),
  KEY `AppId` (`AppId`(191)),
  KEY `ClusterName` (`ClusterName`(191)),
  KEY `NamespaceName` (`NamespaceName`(191))
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COMMENT='commit 历史表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Commit`
--

LOCK TABLES `Commit` WRITE;
/*!40000 ALTER TABLE `Commit` DISABLE KEYS */;
INSERT INTO `Commit` VALUES (1,'{\"createItems\":[{\"namespaceId\":2,\"key\":\"content\",\"value\":\"\\u003capp\\u003e\\n    \\u003ctimeout\\u003e100\\u003c/timeout\\u003e\\n\\u003c/app\\u003e\",\"lineNum\":1,\"id\":2,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:28:52\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:28:52\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.xml',NULL,'\0','apollo','2019-12-27 13:28:52','apollo','2019-12-27 13:28:52'),(2,'{\"createItems\":[{\"namespaceId\":3,\"key\":\"content\",\"value\":\"{\\\"timeout\\\": 100}\",\"lineNum\":1,\"id\":3,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:29:54\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:29:54\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.json',NULL,'\0','apollo','2019-12-27 13:29:54','apollo','2019-12-27 13:29:54'),(3,'{\"createItems\":[{\"namespaceId\":4,\"key\":\"content\",\"value\":\"app:\\n    id: 5\\n    timeout: 100\",\"lineNum\":1,\"id\":4,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:30:25\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:30:25\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.yml',NULL,'\0','apollo','2019-12-27 13:30:25','apollo','2019-12-27 13:30:25'),(4,'{\"createItems\":[{\"namespaceId\":5,\"key\":\"content\",\"value\":\"timeout is 100\",\"lineNum\":1,\"id\":5,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:30:36\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:30:36\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.txt',NULL,'\0','apollo','2019-12-27 13:30:37','apollo','2019-12-27 13:30:37');
/*!40000 ALTER TABLE `Commit` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `GrayReleaseRule`
--

DROP TABLE IF EXISTS `GrayReleaseRule`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `GrayReleaseRule` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `AppId` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `ClusterName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'Cluster Name',
  `NamespaceName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'Namespace Name',
  `BranchName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'branch name',
  `Rules` varchar(16000) DEFAULT '[]' COMMENT '灰度规则',
  `ReleaseId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '灰度对应的release',
  `BranchStatus` tinyint(2) DEFAULT '1' COMMENT '灰度分支状态: 0:删除分支,1:正在使用的规则 2：全量发布',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_Namespace` (`AppId`,`ClusterName`,`NamespaceName`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='灰度规则表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `GrayReleaseRule`
--

LOCK TABLES `GrayReleaseRule` WRITE;
/*!40000 ALTER TABLE `GrayReleaseRule` DISABLE KEYS */;
/*!40000 ALTER TABLE `GrayReleaseRule` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Instance`
--

DROP TABLE IF EXISTS `Instance`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Instance` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `AppId` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `ClusterName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'ClusterName',
  `DataCenter` varchar(64) NOT NULL DEFAULT 'default' COMMENT 'Data Center Name',
  `Ip` varchar(32) NOT NULL DEFAULT '' COMMENT 'instance ip',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  UNIQUE KEY `IX_UNIQUE_KEY` (`AppId`,`ClusterName`,`Ip`,`DataCenter`),
  KEY `IX_IP` (`Ip`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COMMENT='使用配置的应用实例';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Instance`
--

LOCK TABLES `Instance` WRITE;
/*!40000 ALTER TABLE `Instance` DISABLE KEYS */;
INSERT INTO `Instance` VALUES (1,'SampleApp','default','','192.168.48.1','2019-12-27 13:26:48','2019-12-27 13:26:48'),(2,'SampleApp','default','','jmjoy-PC','2019-12-27 13:30:48','2019-12-27 13:30:48');
/*!40000 ALTER TABLE `Instance` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `InstanceConfig`
--

DROP TABLE IF EXISTS `InstanceConfig`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `InstanceConfig` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `InstanceId` int(11) unsigned DEFAULT NULL COMMENT 'Instance Id',
  `ConfigAppId` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'Config App Id',
  `ConfigClusterName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'Config Cluster Name',
  `ConfigNamespaceName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'Config Namespace Name',
  `ReleaseKey` varchar(64) NOT NULL DEFAULT '' COMMENT '发布的Key',
  `ReleaseDeliveryTime` timestamp NULL DEFAULT NULL COMMENT '配置获取时间',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  UNIQUE KEY `IX_UNIQUE_KEY` (`InstanceId`,`ConfigAppId`,`ConfigNamespaceName`),
  KEY `IX_ReleaseKey` (`ReleaseKey`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_Valid_Namespace` (`ConfigAppId`,`ConfigClusterName`,`ConfigNamespaceName`,`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COMMENT='应用实例的配置信息';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `InstanceConfig`
--

LOCK TABLES `InstanceConfig` WRITE;
/*!40000 ALTER TABLE `InstanceConfig` DISABLE KEYS */;
INSERT INTO `InstanceConfig` VALUES (1,1,'SampleApp','default','application','20161009155425-d3a0749c6e20bc15','2019-12-27 13:26:47','2019-12-27 13:26:47','2019-12-27 13:26:47'),(2,1,'SampleApp','default','application.yml','20191227213027-b2af1169c4a6ff5d','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(3,1,'SampleApp','default','application.json','20191227212956-0b3b1169c4a6ff5c','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(4,1,'SampleApp','default','application.xml','20191227212853-aeee1169c4a6ff5b','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(5,1,'SampleApp','default','application.txt','20191227213038-a1471169c4a6ff5e','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(6,2,'SampleApp','default','application','20161009155425-d3a0749c6e20bc15','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47');
/*!40000 ALTER TABLE `InstanceConfig` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Item`
--

DROP TABLE IF EXISTS `Item`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Item` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `NamespaceId` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '集群NamespaceId',
  `Key` varchar(128) NOT NULL DEFAULT 'default' COMMENT '配置项Key',
  `Value` longtext NOT NULL COMMENT '配置项值',
  `Comment` varchar(1024) DEFAULT '' COMMENT '注释',
  `LineNum` int(10) unsigned DEFAULT '0' COMMENT '行号',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_GroupId` (`NamespaceId`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COMMENT='配置项目';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Item`
--

LOCK TABLES `Item` WRITE;
/*!40000 ALTER TABLE `Item` DISABLE KEYS */;
INSERT INTO `Item` VALUES (1,1,'timeout','100','sample timeout配置',1,'\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,2,'content','<app>\n    <timeout>100</timeout>\n</app>',NULL,1,'\0','apollo','2019-12-27 13:28:52','apollo','2019-12-27 13:28:52'),(3,3,'content','{\"timeout\": 100}',NULL,1,'\0','apollo','2019-12-27 13:29:54','apollo','2019-12-27 13:29:54'),(4,4,'content','app:\n    id: 5\n    timeout: 100',NULL,1,'\0','apollo','2019-12-27 13:30:25','apollo','2019-12-27 13:30:25'),(5,5,'content','timeout is 100',NULL,1,'\0','apollo','2019-12-27 13:30:37','apollo','2019-12-27 13:30:37');
/*!40000 ALTER TABLE `Item` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Namespace`
--

DROP TABLE IF EXISTS `Namespace`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Namespace` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `AppId` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `ClusterName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'Cluster Name',
  `NamespaceName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'Namespace Name',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `AppId_ClusterName_NamespaceName` (`AppId`(191),`ClusterName`(191),`NamespaceName`(191)),
  KEY `DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_NamespaceName` (`NamespaceName`(191))
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COMMENT='命名空间';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Namespace`
--

LOCK TABLES `Namespace` WRITE;
/*!40000 ALTER TABLE `Namespace` DISABLE KEYS */;
INSERT INTO `Namespace` VALUES (1,'SampleApp','default','application','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'SampleApp','default','application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(3,'SampleApp','default','application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(4,'SampleApp','default','application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(5,'SampleApp','default','application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12');
/*!40000 ALTER TABLE `Namespace` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `NamespaceLock`
--

DROP TABLE IF EXISTS `NamespaceLock`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `NamespaceLock` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增id',
  `NamespaceId` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '集群NamespaceId',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT 'default' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  `IsDeleted` bit(1) DEFAULT b'0' COMMENT '软删除',
  PRIMARY KEY (`Id`),
  UNIQUE KEY `IX_NamespaceId` (`NamespaceId`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='namespace的编辑锁';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `NamespaceLock`
--

LOCK TABLES `NamespaceLock` WRITE;
/*!40000 ALTER TABLE `NamespaceLock` DISABLE KEYS */;
/*!40000 ALTER TABLE `NamespaceLock` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Release`
--

DROP TABLE IF EXISTS `Release`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Release` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `ReleaseKey` varchar(64) NOT NULL DEFAULT '' COMMENT '发布的Key',
  `Name` varchar(64) NOT NULL DEFAULT 'default' COMMENT '发布名字',
  `Comment` varchar(256) DEFAULT NULL COMMENT '发布说明',
  `AppId` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `ClusterName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ClusterName',
  `NamespaceName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'namespaceName',
  `Configurations` longtext NOT NULL COMMENT '发布配置',
  `IsAbandoned` bit(1) NOT NULL DEFAULT b'0' COMMENT '是否废弃',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `AppId_ClusterName_GroupName` (`AppId`(191),`ClusterName`(191),`NamespaceName`(191)),
  KEY `DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_ReleaseKey` (`ReleaseKey`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COMMENT='发布';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Release`
--

LOCK TABLES `Release` WRITE;
/*!40000 ALTER TABLE `Release` DISABLE KEYS */;
INSERT INTO `Release` VALUES (1,'20161009155425-d3a0749c6e20bc15','20161009155424-release','Sample发布','SampleApp','default','application','{\"timeout\":\"100\"}','\0','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'20191227212853-aeee1169c4a6ff5b','20191227212852-release','','SampleApp','default','application.xml','{\"content\":\"\\u003capp\\u003e\\n    \\u003ctimeout\\u003e100\\u003c/timeout\\u003e\\n\\u003c/app\\u003e\"}','\0','\0','apollo','2019-12-27 13:28:54','apollo','2019-12-27 13:28:54'),(3,'20191227212956-0b3b1169c4a6ff5c','20191227212955-release','','SampleApp','default','application.json','{\"content\":\"{\\\"timeout\\\": 100}\"}','\0','\0','apollo','2019-12-27 13:29:56','apollo','2019-12-27 13:29:56'),(4,'20191227213027-b2af1169c4a6ff5d','20191227213026-release','','SampleApp','default','application.yml','{\"content\":\"app:\\n    id: 5\\n    timeout: 100\"}','\0','\0','apollo','2019-12-27 13:30:27','apollo','2019-12-27 13:30:27'),(5,'20191227213038-a1471169c4a6ff5e','20191227213037-release','','SampleApp','default','application.txt','{\"content\":\"timeout is 100\"}','\0','\0','apollo','2019-12-27 13:30:38','apollo','2019-12-27 13:30:38');
/*!40000 ALTER TABLE `Release` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ReleaseHistory`
--

DROP TABLE IF EXISTS `ReleaseHistory`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ReleaseHistory` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `AppId` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `ClusterName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'ClusterName',
  `NamespaceName` varchar(32) NOT NULL DEFAULT 'default' COMMENT 'namespaceName',
  `BranchName` varchar(32) NOT NULL DEFAULT 'default' COMMENT '发布分支名',
  `ReleaseId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '关联的Release Id',
  `PreviousReleaseId` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '前一次发布的ReleaseId',
  `Operation` tinyint(3) unsigned NOT NULL DEFAULT '0' COMMENT '发布类型，0: 普通发布，1: 回滚，2: 灰度发布，3: 灰度规则更新，4: 灰度合并回主分支发布，5: 主分支发布灰度自动发布，6: 主分支回滚灰度自动发布，7: 放弃灰度',
  `OperationContext` longtext NOT NULL COMMENT '发布上下文信息',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_Namespace` (`AppId`,`ClusterName`,`NamespaceName`,`BranchName`),
  KEY `IX_ReleaseId` (`ReleaseId`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COMMENT='发布历史';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ReleaseHistory`
--

LOCK TABLES `ReleaseHistory` WRITE;
/*!40000 ALTER TABLE `ReleaseHistory` DISABLE KEYS */;
INSERT INTO `ReleaseHistory` VALUES (1,'SampleApp','default','application','default',1,0,0,'{}','\0','apollo','2019-12-27 13:12:54','apollo','2019-12-27 13:12:54'),(2,'SampleApp','default','application.xml','default',2,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:28:54','apollo','2019-12-27 13:28:54'),(3,'SampleApp','default','application.json','default',3,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:29:56','apollo','2019-12-27 13:29:56'),(4,'SampleApp','default','application.yml','default',4,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:30:27','apollo','2019-12-27 13:30:27'),(5,'SampleApp','default','application.txt','default',5,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:30:38','apollo','2019-12-27 13:30:38');
/*!40000 ALTER TABLE `ReleaseHistory` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ReleaseMessage`
--

DROP TABLE IF EXISTS `ReleaseMessage`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ReleaseMessage` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `Message` varchar(1024) NOT NULL DEFAULT '' COMMENT '发布的消息内容',
  `DataChange_LastTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_Message` (`Message`(191))
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COMMENT='发布消息';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ReleaseMessage`
--

LOCK TABLES `ReleaseMessage` WRITE;
/*!40000 ALTER TABLE `ReleaseMessage` DISABLE KEYS */;
INSERT INTO `ReleaseMessage` VALUES (1,'SampleApp+default+application.xml','2019-12-27 13:28:54'),(2,'SampleApp+default+application.json','2019-12-27 13:29:56'),(3,'SampleApp+default+application.yml','2019-12-27 13:30:27'),(4,'SampleApp+default+application.txt','2019-12-27 13:30:38');
/*!40000 ALTER TABLE `ReleaseMessage` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ServerConfig`
--

DROP TABLE IF EXISTS `ServerConfig`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ServerConfig` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `Key` varchar(64) NOT NULL DEFAULT 'default' COMMENT '配置项Key',
  `Cluster` varchar(32) NOT NULL DEFAULT 'default' COMMENT '配置对应的集群，default为不针对特定的集群',
  `Value` varchar(2048) NOT NULL DEFAULT 'default' COMMENT '配置项值',
  `Comment` varchar(1024) DEFAULT '' COMMENT '注释',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_Key` (`Key`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COMMENT='配置服务自身配置';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ServerConfig`
--

LOCK TABLES `ServerConfig` WRITE;
/*!40000 ALTER TABLE `ServerConfig` DISABLE KEYS */;
INSERT INTO `ServerConfig` VALUES (1,'eureka.service.url','default','http://localhost:8080/eureka/','Eureka服务Url，多个service以英文逗号分隔','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'namespace.lock.switch','default','false','一次发布只能有一个人修改开关','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'item.value.length.limit','default','20000','item value最大长度限制','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'config-service.cache.enabled','default','false','ConfigService是否开启缓存，开启后能提高性能，但是会增大内存消耗！','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(5,'item.key.length.limit','default','128','item key 最大长度限制','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54');
/*!40000 ALTER TABLE `ServerConfig` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Current Database: `ApolloPortalDB`
--

CREATE DATABASE /*!32312 IF NOT EXISTS*/ `ApolloPortalDB` /*!40100 DEFAULT CHARACTER SET utf8mb4 */;

USE `ApolloPortalDB`;

--
-- Table structure for table `App`
--

DROP TABLE IF EXISTS `App`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `App` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `AppId` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `Name` varchar(500) NOT NULL DEFAULT 'default' COMMENT '应用名',
  `OrgId` varchar(32) NOT NULL DEFAULT 'default' COMMENT '部门Id',
  `OrgName` varchar(64) NOT NULL DEFAULT 'default' COMMENT '部门名字',
  `OwnerName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ownerName',
  `OwnerEmail` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ownerEmail',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `AppId` (`AppId`(191)),
  KEY `DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_Name` (`Name`(191))
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COMMENT='应用表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `App`
--

LOCK TABLES `App` WRITE;
/*!40000 ALTER TABLE `App` DISABLE KEYS */;
INSERT INTO `App` VALUES (1,'SampleApp','Sample App','TEST1','样例部门1','apollo','apollo@acme.com','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54');
/*!40000 ALTER TABLE `App` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `AppNamespace`
--

DROP TABLE IF EXISTS `AppNamespace`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `AppNamespace` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `Name` varchar(32) NOT NULL DEFAULT '' COMMENT 'namespace名字，注意，需要全局唯一',
  `AppId` varchar(32) NOT NULL DEFAULT '' COMMENT 'app id',
  `Format` varchar(32) NOT NULL DEFAULT 'properties' COMMENT 'namespace的format类型',
  `IsPublic` bit(1) NOT NULL DEFAULT b'0' COMMENT 'namespace是否为公共',
  `Comment` varchar(64) NOT NULL DEFAULT '' COMMENT '注释',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT '' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_AppId` (`AppId`),
  KEY `Name_AppId` (`Name`,`AppId`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COMMENT='应用namespace定义';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `AppNamespace`
--

LOCK TABLES `AppNamespace` WRITE;
/*!40000 ALTER TABLE `AppNamespace` DISABLE KEYS */;
INSERT INTO `AppNamespace` VALUES (1,'application','SampleApp','properties','\0','default app namespace','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'application.xml','SampleApp','xml','\0','','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(3,'application.json','SampleApp','json','\0','','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(4,'application.yml','SampleApp','yml','\0','','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(5,'application.txt','SampleApp','txt','\0','','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12');
/*!40000 ALTER TABLE `AppNamespace` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Authorities`
--

DROP TABLE IF EXISTS `Authorities`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Authorities` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `Username` varchar(64) NOT NULL,
  `Authority` varchar(50) NOT NULL,
  PRIMARY KEY (`Id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Authorities`
--

LOCK TABLES `Authorities` WRITE;
/*!40000 ALTER TABLE `Authorities` DISABLE KEYS */;
INSERT INTO `Authorities` VALUES (1,'apollo','ROLE_user');
/*!40000 ALTER TABLE `Authorities` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Consumer`
--

DROP TABLE IF EXISTS `Consumer`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Consumer` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `AppId` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `Name` varchar(500) NOT NULL DEFAULT 'default' COMMENT '应用名',
  `OrgId` varchar(32) NOT NULL DEFAULT 'default' COMMENT '部门Id',
  `OrgName` varchar(64) NOT NULL DEFAULT 'default' COMMENT '部门名字',
  `OwnerName` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ownerName',
  `OwnerEmail` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'ownerEmail',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `AppId` (`AppId`(191)),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='开放API消费者';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Consumer`
--

LOCK TABLES `Consumer` WRITE;
/*!40000 ALTER TABLE `Consumer` DISABLE KEYS */;
/*!40000 ALTER TABLE `Consumer` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ConsumerAudit`
--

DROP TABLE IF EXISTS `ConsumerAudit`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ConsumerAudit` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `ConsumerId` int(11) unsigned DEFAULT NULL COMMENT 'Consumer Id',
  `Uri` varchar(1024) NOT NULL DEFAULT '' COMMENT '访问的Uri',
  `Method` varchar(16) NOT NULL DEFAULT '' COMMENT '访问的Method',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_ConsumerId` (`ConsumerId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='consumer审计表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ConsumerAudit`
--

LOCK TABLES `ConsumerAudit` WRITE;
/*!40000 ALTER TABLE `ConsumerAudit` DISABLE KEYS */;
/*!40000 ALTER TABLE `ConsumerAudit` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ConsumerRole`
--

DROP TABLE IF EXISTS `ConsumerRole`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ConsumerRole` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `ConsumerId` int(11) unsigned DEFAULT NULL COMMENT 'Consumer Id',
  `RoleId` int(10) unsigned DEFAULT NULL COMMENT 'Role Id',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) DEFAULT '' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_RoleId` (`RoleId`),
  KEY `IX_ConsumerId_RoleId` (`ConsumerId`,`RoleId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='consumer和role的绑定表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ConsumerRole`
--

LOCK TABLES `ConsumerRole` WRITE;
/*!40000 ALTER TABLE `ConsumerRole` DISABLE KEYS */;
/*!40000 ALTER TABLE `ConsumerRole` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ConsumerToken`
--

DROP TABLE IF EXISTS `ConsumerToken`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ConsumerToken` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `ConsumerId` int(11) unsigned DEFAULT NULL COMMENT 'ConsumerId',
  `Token` varchar(128) NOT NULL DEFAULT '' COMMENT 'token',
  `Expires` datetime NOT NULL DEFAULT '2099-01-01 00:00:00' COMMENT 'token失效时间',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  UNIQUE KEY `IX_Token` (`Token`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='consumer token表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ConsumerToken`
--

LOCK TABLES `ConsumerToken` WRITE;
/*!40000 ALTER TABLE `ConsumerToken` DISABLE KEYS */;
/*!40000 ALTER TABLE `ConsumerToken` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Favorite`
--

DROP TABLE IF EXISTS `Favorite`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Favorite` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `UserId` varchar(32) NOT NULL DEFAULT 'default' COMMENT '收藏的用户',
  `AppId` varchar(500) NOT NULL DEFAULT 'default' COMMENT 'AppID',
  `Position` int(32) NOT NULL DEFAULT '10000' COMMENT '收藏顺序',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `AppId` (`AppId`(191)),
  KEY `IX_UserId` (`UserId`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='应用收藏表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Favorite`
--

LOCK TABLES `Favorite` WRITE;
/*!40000 ALTER TABLE `Favorite` DISABLE KEYS */;
/*!40000 ALTER TABLE `Favorite` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Permission`
--

DROP TABLE IF EXISTS `Permission`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Permission` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `PermissionType` varchar(32) NOT NULL DEFAULT '' COMMENT '权限类型',
  `TargetId` varchar(256) NOT NULL DEFAULT '' COMMENT '权限对象类型',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT '' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_TargetId_PermissionType` (`TargetId`(191),`PermissionType`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=23 DEFAULT CHARSET=utf8mb4 COMMENT='permission表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Permission`
--

LOCK TABLES `Permission` WRITE;
/*!40000 ALTER TABLE `Permission` DISABLE KEYS */;
INSERT INTO `Permission` VALUES (1,'CreateCluster','SampleApp','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'CreateNamespace','SampleApp','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'AssignRole','SampleApp','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'ModifyNamespace','SampleApp+application','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(5,'ReleaseNamespace','SampleApp+application','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(6,'CreateApplication','SystemRole','\0','apollo','2019-12-27 13:23:32','apollo','2019-12-27 13:23:32'),(7,'ModifyNamespace','SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(8,'ReleaseNamespace','SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(9,'ModifyNamespace','SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(10,'ReleaseNamespace','SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(11,'ModifyNamespace','SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(12,'ReleaseNamespace','SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(13,'ModifyNamespace','SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(14,'ReleaseNamespace','SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(15,'ModifyNamespace','SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(16,'ReleaseNamespace','SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(17,'ModifyNamespace','SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(18,'ReleaseNamespace','SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(19,'ModifyNamespace','SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(20,'ReleaseNamespace','SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(21,'ModifyNamespace','SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(22,'ReleaseNamespace','SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12');
/*!40000 ALTER TABLE `Permission` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Role`
--

DROP TABLE IF EXISTS `Role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Role` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `RoleName` varchar(256) NOT NULL DEFAULT '' COMMENT 'Role name',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_RoleName` (`RoleName`(191)),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COMMENT='角色表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Role`
--

LOCK TABLES `Role` WRITE;
/*!40000 ALTER TABLE `Role` DISABLE KEYS */;
INSERT INTO `Role` VALUES (1,'Master+SampleApp','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'ModifyNamespace+SampleApp+application','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'ReleaseNamespace+SampleApp+application','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'CreateApplication+SystemRole','\0','apollo','2019-12-27 13:23:32','apollo','2019-12-27 13:23:32'),(5,'ModifyNamespace+SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(6,'ReleaseNamespace+SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(7,'ModifyNamespace+SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(8,'ReleaseNamespace+SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(9,'ModifyNamespace+SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(10,'ReleaseNamespace+SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(11,'ModifyNamespace+SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(12,'ReleaseNamespace+SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(13,'ModifyNamespace+SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(14,'ReleaseNamespace+SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(15,'ModifyNamespace+SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(16,'ReleaseNamespace+SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(17,'ModifyNamespace+SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(18,'ReleaseNamespace+SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(19,'ModifyNamespace+SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(20,'ReleaseNamespace+SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12');
/*!40000 ALTER TABLE `Role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `RolePermission`
--

DROP TABLE IF EXISTS `RolePermission`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `RolePermission` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `RoleId` int(10) unsigned DEFAULT NULL COMMENT 'Role Id',
  `PermissionId` int(10) unsigned DEFAULT NULL COMMENT 'Permission Id',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) DEFAULT '' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_RoleId` (`RoleId`),
  KEY `IX_PermissionId` (`PermissionId`)
) ENGINE=InnoDB AUTO_INCREMENT=23 DEFAULT CHARSET=utf8mb4 COMMENT='角色和权限的绑定表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `RolePermission`
--

LOCK TABLES `RolePermission` WRITE;
/*!40000 ALTER TABLE `RolePermission` DISABLE KEYS */;
INSERT INTO `RolePermission` VALUES (1,1,1,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,1,2,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,1,3,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,2,4,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(5,3,5,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(6,4,6,'\0','apollo','2019-12-27 13:23:32','apollo','2019-12-27 13:23:32'),(7,5,7,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(8,6,8,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(9,7,9,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(10,8,10,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(11,9,11,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(12,10,12,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(13,11,13,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(14,12,14,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(15,13,15,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(16,14,16,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(17,15,17,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(18,16,18,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(19,17,19,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(20,18,20,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(21,19,21,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(22,20,22,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12');
/*!40000 ALTER TABLE `RolePermission` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ServerConfig`
--

DROP TABLE IF EXISTS `ServerConfig`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ServerConfig` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `Key` varchar(64) NOT NULL DEFAULT 'default' COMMENT '配置项Key',
  `Value` varchar(2048) NOT NULL DEFAULT 'default' COMMENT '配置项值',
  `Comment` varchar(1024) DEFAULT '' COMMENT '注释',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) NOT NULL DEFAULT 'default' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_Key` (`Key`),
  KEY `DataChange_LastTime` (`DataChange_LastTime`)
) ENGINE=InnoDB AUTO_INCREMENT=8 DEFAULT CHARSET=utf8mb4 COMMENT='配置服务自身配置';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ServerConfig`
--

LOCK TABLES `ServerConfig` WRITE;
/*!40000 ALTER TABLE `ServerConfig` DISABLE KEYS */;
INSERT INTO `ServerConfig` VALUES (1,'apollo.portal.envs','dev','可支持的环境列表','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'organizations','[{\"orgId\":\"TEST1\",\"orgName\":\"样例部门1\"},{\"orgId\":\"TEST2\",\"orgName\":\"样例部门2\"}]','部门列表','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'superAdmin','apollo','Portal超级管理员','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'api.readTimeout','10000','http接口read timeout','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(5,'consumer.token.salt','someSalt','consumer token salt','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(6,'admin.createPrivateNamespace.switch','true','是否允许项目管理员创建私有namespace','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(7,'configView.memberOnly.envs','dev','只对项目成员显示配置信息的环境列表，多个env以英文逗号分隔','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54');
/*!40000 ALTER TABLE `ServerConfig` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `UserRole`
--

DROP TABLE IF EXISTS `UserRole`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `UserRole` (
  `Id` int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `UserId` varchar(128) DEFAULT '' COMMENT '用户身份标识',
  `RoleId` int(10) unsigned DEFAULT NULL COMMENT 'Role Id',
  `IsDeleted` bit(1) NOT NULL DEFAULT b'0' COMMENT '1: deleted, 0: normal',
  `DataChange_CreatedBy` varchar(32) DEFAULT '' COMMENT '创建人邮箱前缀',
  `DataChange_CreatedTime` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `DataChange_LastModifiedBy` varchar(32) DEFAULT '' COMMENT '最后修改人邮箱前缀',
  `DataChange_LastTime` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '最后修改时间',
  PRIMARY KEY (`Id`),
  KEY `IX_DataChange_LastTime` (`DataChange_LastTime`),
  KEY `IX_RoleId` (`RoleId`),
  KEY `IX_UserId_RoleId` (`UserId`,`RoleId`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COMMENT='用户和role的绑定表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `UserRole`
--

LOCK TABLES `UserRole` WRITE;
/*!40000 ALTER TABLE `UserRole` DISABLE KEYS */;
INSERT INTO `UserRole` VALUES (1,'apollo',1,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'apollo',2,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'apollo',3,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'apollo',5,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(5,'apollo',6,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(6,'apollo',9,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(7,'apollo',10,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(8,'apollo',13,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(9,'apollo',14,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(10,'apollo',17,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(11,'apollo',18,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12');
/*!40000 ALTER TABLE `UserRole` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Users`
--

DROP TABLE IF EXISTS `Users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Users` (
  `Id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增Id',
  `Username` varchar(64) NOT NULL DEFAULT 'default' COMMENT '用户名',
  `Password` varchar(64) NOT NULL DEFAULT 'default' COMMENT '密码',
  `Email` varchar(64) NOT NULL DEFAULT 'default' COMMENT '邮箱地址',
  `Enabled` tinyint(4) DEFAULT NULL COMMENT '是否有效',
  PRIMARY KEY (`Id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COMMENT='用户表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Users`
--

LOCK TABLES `Users` WRITE;
/*!40000 ALTER TABLE `Users` DISABLE KEYS */;
INSERT INTO `Users` VALUES (1,'apollo','$2a$10$7r20uS.BQ9uBpf3Baj3uQOZvMVvB1RN3PYoKE94gtz2.WAOuiiwXS','apollo@acme.com',1);
/*!40000 ALTER TABLE `Users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2019-12-27 21:32:00
