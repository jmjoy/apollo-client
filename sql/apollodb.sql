-- MySQL dump 10.18  Distrib 10.3.27-MariaDB, for debian-linux-gnu (x86_64)
--
-- Host: 127.0.0.1    Database: ApolloConfigDB
-- ------------------------------------------------------
-- Server version	5.7.34

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
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
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COMMENT='应用表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `App`
--

LOCK TABLES `App` WRITE;
/*!40000 ALTER TABLE `App` DISABLE KEYS */;
INSERT INTO `App` VALUES (1,'SampleApp','Sample App','TEST1','样例部门1','apollo','apollo@acme.com','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'TestApp1','TestApp1','TEST1','样例部门1','apollo','apollo@acme.com','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(3,'TestApp2','TestApp2','TEST1','样例部门1','apollo','apollo@acme.com','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43');
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
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COMMENT='应用namespace定义';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `AppNamespace`
--

LOCK TABLES `AppNamespace` WRITE;
/*!40000 ALTER TABLE `AppNamespace` DISABLE KEYS */;
INSERT INTO `AppNamespace` VALUES (1,'application','SampleApp','properties','\0','default app namespace','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'application.xml','SampleApp','xml','\0','','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(3,'application.json','SampleApp','json','\0','','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(4,'application.yml','SampleApp','yml','\0','','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(5,'application.txt','SampleApp','txt','\0','','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(6,'application','TestApp1','properties','\0','default app namespace','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(7,'foo.yml','TestApp1','yml','\0','','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(8,'foo1','TestApp1','properties','\0','','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(9,'foo2','TestApp1','properties','\0','','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(10,'application','TestApp2','properties','\0','default app namespace','\0','apollo','2021-06-28 06:49:44','apollo','2021-06-28 06:49:44'),(11,'watcher','TestApp2','properties','\0','','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(12,'watcher2.json','TestApp2','json','\0','','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54');
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
) ENGINE=InnoDB AUTO_INCREMENT=104 DEFAULT CHARSET=utf8mb4 COMMENT='日志审计表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Audit`
--

LOCK TABLES `Audit` WRITE;
/*!40000 ALTER TABLE `Audit` DISABLE KEYS */;
INSERT INTO `Audit` VALUES (1,'Namespace',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:14',NULL,'2019-12-27 13:27:14'),(2,'AppNamespace',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:14',NULL,'2019-12-27 13:27:14'),(3,'Namespace',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:26',NULL,'2019-12-27 13:27:26'),(4,'AppNamespace',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:27:26',NULL,'2019-12-27 13:27:26'),(5,'Namespace',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:03',NULL,'2019-12-27 13:28:03'),(6,'AppNamespace',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:03',NULL,'2019-12-27 13:28:03'),(7,'Namespace',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:12',NULL,'2019-12-27 13:28:12'),(8,'AppNamespace',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:12',NULL,'2019-12-27 13:28:12'),(9,'Item',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:52',NULL,'2019-12-27 13:28:52'),(10,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:52',NULL,'2019-12-27 13:28:52'),(11,'Release',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:54',NULL,'2019-12-27 13:28:54'),(12,'ReleaseHistory',2,'INSERT',NULL,'\0','apollo','2019-12-27 13:28:54',NULL,'2019-12-27 13:28:54'),(13,'Item',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:54',NULL,'2019-12-27 13:29:54'),(14,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:54',NULL,'2019-12-27 13:29:54'),(15,'Release',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:56',NULL,'2019-12-27 13:29:56'),(16,'ReleaseHistory',3,'INSERT',NULL,'\0','apollo','2019-12-27 13:29:56',NULL,'2019-12-27 13:29:56'),(17,'Item',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:25',NULL,'2019-12-27 13:30:25'),(18,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:25',NULL,'2019-12-27 13:30:25'),(19,'Release',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:27',NULL,'2019-12-27 13:30:27'),(20,'ReleaseHistory',4,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:27',NULL,'2019-12-27 13:30:27'),(21,'Item',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:37',NULL,'2019-12-27 13:30:37'),(22,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:37',NULL,'2019-12-27 13:30:37'),(23,'Release',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:38',NULL,'2019-12-27 13:30:38'),(24,'ReleaseHistory',5,'INSERT',NULL,'\0','apollo','2019-12-27 13:30:38',NULL,'2019-12-27 13:30:38'),(25,'App',2,'INSERT',NULL,'\0','apollo','2021-06-26 01:13:44',NULL,'2021-06-26 01:13:44'),(26,'AppNamespace',6,'INSERT',NULL,'\0','apollo','2021-06-26 01:13:44',NULL,'2021-06-26 01:13:44'),(27,'Cluster',2,'INSERT',NULL,'\0','apollo','2021-06-26 01:13:44',NULL,'2021-06-26 01:13:44'),(28,'Namespace',6,'INSERT',NULL,'\0','apollo','2021-06-26 01:13:44',NULL,'2021-06-26 01:13:44'),(29,'Item',6,'INSERT',NULL,'\0','apollo','2021-06-26 01:13:59',NULL,'2021-06-26 01:13:59'),(30,'Item',7,'INSERT',NULL,'\0','apollo','2021-06-26 01:14:09',NULL,'2021-06-26 01:14:09'),(31,'Release',6,'INSERT',NULL,'\0','apollo','2021-06-26 01:14:12',NULL,'2021-06-26 01:14:12'),(32,'ReleaseHistory',6,'INSERT',NULL,'\0','apollo','2021-06-26 01:14:12',NULL,'2021-06-26 01:14:12'),(33,'Item',8,'INSERT',NULL,'\0','apollo','2021-06-26 01:14:57',NULL,'2021-06-26 01:14:57'),(34,'Release',7,'INSERT',NULL,'\0','apollo','2021-06-26 01:15:01',NULL,'2021-06-26 01:15:01'),(35,'ReleaseHistory',7,'INSERT',NULL,'\0','apollo','2021-06-26 01:15:01',NULL,'2021-06-26 01:15:01'),(36,'Item',8,'UPDATE',NULL,'\0','apollo','2021-06-26 08:29:45',NULL,'2021-06-26 08:29:45'),(37,'Release',8,'INSERT',NULL,'\0','apollo','2021-06-26 18:33:36',NULL,'2021-06-26 18:33:36'),(38,'ReleaseHistory',8,'INSERT',NULL,'\0','apollo','2021-06-26 18:33:36',NULL,'2021-06-26 18:33:36'),(39,'Namespace',7,'INSERT',NULL,'\0','apollo','2021-06-27 02:50:57',NULL,'2021-06-27 02:50:57'),(40,'AppNamespace',7,'INSERT',NULL,'\0','apollo','2021-06-27 02:50:57',NULL,'2021-06-27 02:50:57'),(41,'Item',9,'INSERT',NULL,'\0','apollo','2021-06-27 02:52:04',NULL,'2021-06-27 02:52:04'),(42,'ItemSet',NULL,'INSERT',NULL,'\0','apollo','2021-06-27 02:52:04',NULL,'2021-06-27 02:52:04'),(43,'Release',9,'INSERT',NULL,'\0','apollo','2021-06-27 02:52:08',NULL,'2021-06-27 02:52:08'),(44,'ReleaseHistory',9,'INSERT',NULL,'\0','apollo','2021-06-27 02:52:08',NULL,'2021-06-27 02:52:08'),(45,'Namespace',8,'INSERT',NULL,'\0','apollo','2021-06-27 02:54:19',NULL,'2021-06-27 02:54:19'),(46,'AppNamespace',8,'INSERT',NULL,'\0','apollo','2021-06-27 02:54:19',NULL,'2021-06-27 02:54:19'),(47,'Item',10,'INSERT',NULL,'\0','apollo','2021-06-27 02:55:47',NULL,'2021-06-27 02:55:47'),(48,'Release',10,'INSERT',NULL,'\0','apollo','2021-06-27 02:55:48',NULL,'2021-06-27 02:55:48'),(49,'ReleaseHistory',10,'INSERT',NULL,'\0','apollo','2021-06-27 02:55:48',NULL,'2021-06-27 02:55:48'),(50,'Namespace',9,'INSERT',NULL,'\0','apollo','2021-06-27 02:56:05',NULL,'2021-06-27 02:56:05'),(51,'AppNamespace',9,'INSERT',NULL,'\0','apollo','2021-06-27 02:56:05',NULL,'2021-06-27 02:56:05'),(52,'Item',11,'INSERT',NULL,'\0','apollo','2021-06-27 02:56:15',NULL,'2021-06-27 02:56:15'),(53,'Release',11,'INSERT',NULL,'\0','apollo','2021-06-27 02:56:16',NULL,'2021-06-27 02:56:16'),(54,'ReleaseHistory',11,'INSERT',NULL,'\0','apollo','2021-06-27 02:56:16',NULL,'2021-06-27 02:56:16'),(55,'Item',10,'DELETE',NULL,'\0','apollo','2021-06-27 12:33:58',NULL,'2021-06-27 12:33:58'),(56,'Item',12,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:06',NULL,'2021-06-27 12:34:06'),(57,'Release',12,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:09',NULL,'2021-06-27 12:34:09'),(58,'ReleaseHistory',12,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:09',NULL,'2021-06-27 12:34:09'),(59,'Item',11,'DELETE',NULL,'\0','apollo','2021-06-27 12:34:13',NULL,'2021-06-27 12:34:13'),(60,'Item',13,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:19',NULL,'2021-06-27 12:34:19'),(61,'Release',13,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:21',NULL,'2021-06-27 12:34:21'),(62,'ReleaseHistory',13,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:21',NULL,'2021-06-27 12:34:21'),(63,'Item',6,'DELETE',NULL,'\0','apollo','2021-06-27 12:34:31',NULL,'2021-06-27 12:34:31'),(64,'Item',7,'DELETE',NULL,'\0','apollo','2021-06-27 12:34:33',NULL,'2021-06-27 12:34:33'),(65,'Item',8,'DELETE',NULL,'\0','apollo','2021-06-27 12:34:35',NULL,'2021-06-27 12:34:35'),(66,'Release',14,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:37',NULL,'2021-06-27 12:34:37'),(67,'ReleaseHistory',14,'INSERT',NULL,'\0','apollo','2021-06-27 12:34:37',NULL,'2021-06-27 12:34:37'),(68,'App',3,'INSERT',NULL,'\0','apollo','2021-06-28 06:49:44',NULL,'2021-06-28 06:49:44'),(69,'AppNamespace',10,'INSERT',NULL,'\0','apollo','2021-06-28 06:49:44',NULL,'2021-06-28 06:49:44'),(70,'Cluster',3,'INSERT',NULL,'\0','apollo','2021-06-28 06:49:44',NULL,'2021-06-28 06:49:44'),(71,'Namespace',10,'INSERT',NULL,'\0','apollo','2021-06-28 06:49:44',NULL,'2021-06-28 06:49:44'),(72,'Release',15,'INSERT',NULL,'\0','apollo','2021-06-28 08:12:16',NULL,'2021-06-28 08:12:16'),(73,'ReleaseHistory',15,'INSERT',NULL,'\0','apollo','2021-06-28 08:12:16',NULL,'2021-06-28 08:12:16'),(74,'Item',14,'INSERT',NULL,'\0','apollo','2021-06-28 08:22:07',NULL,'2021-06-28 08:22:07'),(75,'Item',14,'DELETE',NULL,'\0','apollo','2021-06-28 08:22:21',NULL,'2021-06-28 08:22:21'),(76,'Item',15,'INSERT',NULL,'\0','apollo','2021-06-28 08:28:19',NULL,'2021-06-28 08:28:19'),(77,'Item',15,'DELETE',NULL,'\0','apollo','2021-06-28 08:33:52',NULL,'2021-06-28 08:33:52'),(78,'Item',16,'INSERT',NULL,'\0','apollo','2021-06-28 08:33:55',NULL,'2021-06-28 08:33:55'),(79,'Item',17,'INSERT',NULL,'\0','apollo','2021-06-28 08:33:55',NULL,'2021-06-28 08:33:55'),(80,'Item',16,'DELETE',NULL,'\0','apollo','2021-06-28 08:36:46',NULL,'2021-06-28 08:36:46'),(81,'Item',17,'DELETE',NULL,'\0','apollo','2021-06-28 08:36:48',NULL,'2021-06-28 08:36:48'),(82,'Item',18,'INSERT',NULL,'\0','apollo','2021-06-28 08:36:51',NULL,'2021-06-28 08:36:51'),(83,'Item',19,'INSERT',NULL,'\0','apollo','2021-06-28 08:36:51',NULL,'2021-06-28 08:36:51'),(84,'Item',18,'DELETE',NULL,'\0','apollo','2021-06-28 15:41:34',NULL,'2021-06-28 15:41:34'),(85,'Item',19,'DELETE',NULL,'\0','apollo','2021-06-28 15:41:36',NULL,'2021-06-28 15:41:36'),(86,'Item',20,'INSERT',NULL,'\0','apollo','2021-06-28 15:41:41',NULL,'2021-06-28 15:41:41'),(87,'Item',21,'INSERT',NULL,'\0','apollo','2021-06-28 15:41:41',NULL,'2021-06-28 15:41:41'),(88,'Release',16,'INSERT',NULL,'\0','apollo','2021-06-28 15:44:05',NULL,'2021-06-28 15:44:05'),(89,'ReleaseHistory',16,'INSERT',NULL,'\0','apollo','2021-06-28 15:44:05',NULL,'2021-06-28 15:44:05'),(90,'Namespace',11,'INSERT',NULL,'\0','apollo','2021-07-03 03:49:06',NULL,'2021-07-03 03:49:06'),(91,'AppNamespace',11,'INSERT',NULL,'\0','apollo','2021-07-03 03:49:06',NULL,'2021-07-03 03:49:06'),(92,'Release',17,'INSERT',NULL,'\0','apollo','2021-07-03 03:49:11',NULL,'2021-07-03 03:49:11'),(93,'ReleaseHistory',17,'INSERT',NULL,'\0','apollo','2021-07-03 03:49:11',NULL,'2021-07-03 03:49:11'),(94,'Namespace',12,'INSERT',NULL,'\0','apollo','2021-07-03 05:34:54',NULL,'2021-07-03 05:34:54'),(95,'AppNamespace',12,'INSERT',NULL,'\0','apollo','2021-07-03 05:34:54',NULL,'2021-07-03 05:34:54'),(96,'Release',18,'INSERT',NULL,'\0','apollo','2021-07-03 05:43:20',NULL,'2021-07-03 05:43:20'),(97,'ReleaseHistory',18,'INSERT',NULL,'\0','apollo','2021-07-03 05:43:20',NULL,'2021-07-03 05:43:20'),(98,'Item',22,'INSERT',NULL,'\0','apollo','2021-07-03 07:13:51',NULL,'2021-07-03 07:13:51'),(99,'Release',19,'INSERT',NULL,'\0','apollo','2021-07-03 07:13:51',NULL,'2021-07-03 07:13:51'),(100,'ReleaseHistory',19,'INSERT',NULL,'\0','apollo','2021-07-03 07:13:51',NULL,'2021-07-03 07:13:51'),(101,'Item',22,'DELETE',NULL,'\0','apollo','2021-07-03 07:14:19',NULL,'2021-07-03 07:14:19'),(102,'Release',20,'INSERT',NULL,'\0','apollo','2021-07-03 07:14:21',NULL,'2021-07-03 07:14:21'),(103,'ReleaseHistory',20,'INSERT',NULL,'\0','apollo','2021-07-03 07:14:21',NULL,'2021-07-03 07:14:21');
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
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COMMENT='集群';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Cluster`
--

LOCK TABLES `Cluster` WRITE;
/*!40000 ALTER TABLE `Cluster` DISABLE KEYS */;
INSERT INTO `Cluster` VALUES (1,'default','SampleApp',0,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'default','TestApp1',0,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(3,'default','TestApp2',0,'\0','apollo','2021-06-28 06:49:44','apollo','2021-06-28 06:49:44');
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
) ENGINE=InnoDB AUTO_INCREMENT=35 DEFAULT CHARSET=utf8mb4 COMMENT='commit 历史表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Commit`
--

LOCK TABLES `Commit` WRITE;
/*!40000 ALTER TABLE `Commit` DISABLE KEYS */;
INSERT INTO `Commit` VALUES (1,'{\"createItems\":[{\"namespaceId\":2,\"key\":\"content\",\"value\":\"\\u003capp\\u003e\\n    \\u003ctimeout\\u003e100\\u003c/timeout\\u003e\\n\\u003c/app\\u003e\",\"lineNum\":1,\"id\":2,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:28:52\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:28:52\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.xml',NULL,'\0','apollo','2019-12-27 13:28:52','apollo','2019-12-27 13:28:52'),(2,'{\"createItems\":[{\"namespaceId\":3,\"key\":\"content\",\"value\":\"{\\\"timeout\\\": 100}\",\"lineNum\":1,\"id\":3,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:29:54\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:29:54\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.json',NULL,'\0','apollo','2019-12-27 13:29:54','apollo','2019-12-27 13:29:54'),(3,'{\"createItems\":[{\"namespaceId\":4,\"key\":\"content\",\"value\":\"app:\\n    id: 5\\n    timeout: 100\",\"lineNum\":1,\"id\":4,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:30:25\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:30:25\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.yml',NULL,'\0','apollo','2019-12-27 13:30:25','apollo','2019-12-27 13:30:25'),(4,'{\"createItems\":[{\"namespaceId\":5,\"key\":\"content\",\"value\":\"timeout is 100\",\"lineNum\":1,\"id\":5,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2019-12-27 21:30:36\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2019-12-27 21:30:36\"}],\"updateItems\":[],\"deleteItems\":[]}','SampleApp','default','application.txt',NULL,'\0','apollo','2019-12-27 13:30:37','apollo','2019-12-27 13:30:37'),(5,'{\"createItems\":[{\"namespaceId\":6,\"key\":\"foo\",\"value\":\"bar\",\"lineNum\":1,\"id\":6,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:13:58\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-26 09:13:58\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','application',NULL,'\0','apollo','2021-06-26 01:13:59','apollo','2021-06-26 01:13:59'),(6,'{\"createItems\":[{\"namespaceId\":6,\"key\":\"foo1\",\"value\":\"bar1\",\"lineNum\":2,\"id\":7,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:14:08\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-26 09:14:08\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','application',NULL,'\0','apollo','2021-06-26 01:14:09','apollo','2021-06-26 01:14:09'),(7,'{\"createItems\":[{\"namespaceId\":6,\"key\":\"foo2\",\"value\":\"bar2\\u003dbar2\",\"lineNum\":3,\"id\":8,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:14:56\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-26 09:14:56\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','application',NULL,'\0','apollo','2021-06-26 01:14:57','apollo','2021-06-26 01:14:57'),(8,'{\"createItems\":[],\"updateItems\":[{\"oldItem\":{\"namespaceId\":6,\"key\":\"foo2\",\"value\":\"bar2\\u003dbar2\",\"lineNum\":3,\"id\":8,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:14:57\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-26 09:14:57\"},\"newItem\":{\"namespaceId\":6,\"key\":\"foo2\",\"value\":\"bar2\\u003dbar2\\u003d\",\"comment\":\"\",\"lineNum\":3,\"id\":8,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:14:57\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-26 16:29:44\"}}],\"deleteItems\":[]}','TestApp1','default','application',NULL,'\0','apollo','2021-06-26 08:29:45','apollo','2021-06-26 08:29:45'),(9,'{\"createItems\":[{\"namespaceId\":7,\"key\":\"content\",\"value\":\"foo:\\n- 1\\n- 2\\n- 3\",\"lineNum\":1,\"id\":9,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-27 10:52:03\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 10:52:03\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','foo.yml',NULL,'\0','apollo','2021-06-27 02:52:04','apollo','2021-06-27 02:52:04'),(10,'{\"createItems\":[{\"namespaceId\":8,\"key\":\"a\",\"value\":\"1\",\"lineNum\":1,\"id\":10,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-27 10:55:46\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 10:55:46\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','foo1',NULL,'\0','apollo','2021-06-27 02:55:47','apollo','2021-06-27 02:55:47'),(11,'{\"createItems\":[{\"namespaceId\":9,\"key\":\"a\",\"value\":\"1\",\"lineNum\":1,\"id\":11,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-27 10:56:14\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 10:56:14\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','foo2',NULL,'\0','apollo','2021-06-27 02:56:15','apollo','2021-06-27 02:56:15'),(12,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":8,\"key\":\"a\",\"value\":\"1\",\"lineNum\":1,\"id\":10,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-27 10:55:47\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 20:33:57\"}]}','TestApp1','default','foo1',NULL,'\0','apollo','2021-06-27 12:33:58','apollo','2021-06-27 12:33:58'),(13,'{\"createItems\":[{\"namespaceId\":8,\"key\":\"foo1\",\"value\":\"bar1\",\"lineNum\":1,\"id\":12,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-27 20:34:05\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 20:34:05\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','foo1',NULL,'\0','apollo','2021-06-27 12:34:06','apollo','2021-06-27 12:34:06'),(14,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":9,\"key\":\"a\",\"value\":\"1\",\"lineNum\":1,\"id\":11,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-27 10:56:15\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 20:34:13\"}]}','TestApp1','default','foo2',NULL,'\0','apollo','2021-06-27 12:34:13','apollo','2021-06-27 12:34:13'),(15,'{\"createItems\":[{\"namespaceId\":9,\"key\":\"foo2\",\"value\":\"bar2\",\"lineNum\":1,\"id\":13,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-27 20:34:19\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 20:34:19\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp1','default','foo2',NULL,'\0','apollo','2021-06-27 12:34:19','apollo','2021-06-27 12:34:19'),(16,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":6,\"key\":\"foo\",\"value\":\"bar\",\"lineNum\":1,\"id\":6,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:13:59\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 20:34:30\"}]}','TestApp1','default','application',NULL,'\0','apollo','2021-06-27 12:34:31','apollo','2021-06-27 12:34:31'),(17,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":6,\"key\":\"foo1\",\"value\":\"bar1\",\"lineNum\":2,\"id\":7,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:14:09\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 20:34:33\"}]}','TestApp1','default','application',NULL,'\0','apollo','2021-06-27 12:34:33','apollo','2021-06-27 12:34:33'),(18,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":6,\"key\":\"foo2\",\"value\":\"bar2\\u003dbar2\\u003d\",\"comment\":\"\",\"lineNum\":3,\"id\":8,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-26 09:14:57\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-27 20:34:34\"}]}','TestApp1','default','application',NULL,'\0','apollo','2021-06-27 12:34:35','apollo','2021-06-27 12:34:35'),(19,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"100\",\"comment\":\"\",\"lineNum\":1,\"id\":14,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:22:06\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:22:06\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:22:07','apollo','2021-06-28 08:22:07'),(20,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"100\",\"comment\":\"\",\"lineNum\":1,\"id\":14,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:22:07\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:22:20\"}]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:22:21','apollo','2021-06-28 08:22:21'),(21,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"3000\",\"lineNum\":1,\"id\":15,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:28:18\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:28:18\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:28:19','apollo','2021-06-28 08:28:19'),(22,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"3000\",\"lineNum\":1,\"id\":15,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:28:19\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:33:51\"}]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:33:52','apollo','2021-06-28 08:33:52'),(23,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"3000\",\"lineNum\":1,\"id\":16,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:33:54\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:33:54\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:33:55','apollo','2021-06-28 08:33:55'),(24,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"connect_timeout\",\"value\":\"100\",\"comment\":\"connect timeout\",\"lineNum\":2,\"id\":17,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:33:54\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:33:54\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:33:55','apollo','2021-06-28 08:33:55'),(25,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"3000\",\"lineNum\":1,\"id\":16,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:33:55\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:36:45\"}]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:36:46','apollo','2021-06-28 08:36:46'),(26,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":10,\"key\":\"connect_timeout\",\"value\":\"100\",\"comment\":\"connect timeout\",\"lineNum\":2,\"id\":17,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:33:55\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:36:47\"}]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:36:48','apollo','2021-06-28 08:36:48'),(27,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"3000\",\"lineNum\":1,\"id\":18,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:36:50\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:36:50\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:36:51','apollo','2021-06-28 08:36:51'),(28,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"connect_timeout\",\"value\":\"100\",\"comment\":\"connect timeout\",\"lineNum\":2,\"id\":19,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:36:50\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 16:36:50\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 08:36:51','apollo','2021-06-28 08:36:51'),(29,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"3000\",\"lineNum\":1,\"id\":18,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:36:51\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 23:41:34\"}]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 15:41:35','apollo','2021-06-28 15:41:35'),(30,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":10,\"key\":\"connect_timeout\",\"value\":\"100\",\"comment\":\"connect timeout\",\"lineNum\":2,\"id\":19,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 16:36:51\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 23:41:36\"}]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 15:41:37','apollo','2021-06-28 15:41:37'),(31,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"timeout\",\"value\":\"3000\",\"lineNum\":1,\"id\":20,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 23:41:40\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 23:41:40\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 15:41:41','apollo','2021-06-28 15:41:41'),(32,'{\"createItems\":[{\"namespaceId\":10,\"key\":\"connect_timeout\",\"value\":\"100\",\"comment\":\"connect timeout\",\"lineNum\":2,\"id\":21,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-06-28 23:41:40\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-06-28 23:41:40\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','application',NULL,'\0','apollo','2021-06-28 15:41:41','apollo','2021-06-28 15:41:41'),(33,'{\"createItems\":[{\"namespaceId\":11,\"key\":\"a\",\"value\":\"1\",\"comment\":\"a comment\",\"lineNum\":1,\"id\":22,\"isDeleted\":false,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-07-03 15:13:50\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-07-03 15:13:50\"}],\"updateItems\":[],\"deleteItems\":[]}','TestApp2','default','watcher',NULL,'\0','apollo','2021-07-03 07:13:51','apollo','2021-07-03 07:13:51'),(34,'{\"createItems\":[],\"updateItems\":[],\"deleteItems\":[{\"namespaceId\":11,\"key\":\"a\",\"value\":\"1\",\"comment\":\"a comment\",\"lineNum\":1,\"id\":22,\"isDeleted\":true,\"dataChangeCreatedBy\":\"apollo\",\"dataChangeCreatedTime\":\"2021-07-03 15:13:51\",\"dataChangeLastModifiedBy\":\"apollo\",\"dataChangeLastModifiedTime\":\"2021-07-03 15:14:18\"}]}','TestApp2','default','watcher',NULL,'\0','apollo','2021-07-03 07:14:19','apollo','2021-07-03 07:14:19');
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
) ENGINE=InnoDB AUTO_INCREMENT=8 DEFAULT CHARSET=utf8mb4 COMMENT='使用配置的应用实例';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Instance`
--

LOCK TABLES `Instance` WRITE;
/*!40000 ALTER TABLE `Instance` DISABLE KEYS */;
INSERT INTO `Instance` VALUES (1,'SampleApp','default','','192.168.48.1','2019-12-27 13:26:48','2019-12-27 13:26:48'),(2,'SampleApp','default','','jmjoy-PC','2019-12-27 13:30:48','2019-12-27 13:30:48'),(3,'SampleApp','default','','172.24.0.1','2021-06-25 06:15:57','2021-06-25 06:15:57'),(4,'SampleApp','default','','test-host-name','2021-06-25 06:16:00','2021-06-25 06:16:00'),(5,'TestApp1','default','','jmjoy-PC','2021-06-26 01:14:33','2021-06-26 01:14:33'),(6,'TestApp1','default','','172.22.0.1','2021-06-26 17:01:05','2021-06-26 17:01:05'),(7,'TestApp2','default','','jmjoy-PC','2021-07-03 05:42:54','2021-07-03 05:42:54');
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
) ENGINE=InnoDB AUTO_INCREMENT=25 DEFAULT CHARSET=utf8mb4 COMMENT='应用实例的配置信息';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `InstanceConfig`
--

LOCK TABLES `InstanceConfig` WRITE;
/*!40000 ALTER TABLE `InstanceConfig` DISABLE KEYS */;
INSERT INTO `InstanceConfig` VALUES (1,1,'SampleApp','default','application','20161009155425-d3a0749c6e20bc15','2019-12-27 13:26:47','2019-12-27 13:26:47','2019-12-27 13:26:47'),(2,1,'SampleApp','default','application.yml','20191227213027-b2af1169c4a6ff5d','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(3,1,'SampleApp','default','application.json','20191227212956-0b3b1169c4a6ff5c','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(4,1,'SampleApp','default','application.xml','20191227212853-aeee1169c4a6ff5b','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(5,1,'SampleApp','default','application.txt','20191227213038-a1471169c4a6ff5e','2019-12-27 13:30:47','2019-12-27 13:30:47','2019-12-27 13:30:47'),(6,2,'SampleApp','default','application','20161009155425-d3a0749c6e20bc15','2019-12-27 13:30:47','2019-12-27 13:30:47','2021-07-03 05:41:56'),(7,3,'SampleApp','default','application.yml','20191227213027-b2af1169c4a6ff5d','2021-06-25 06:15:56','2021-06-25 06:15:56','2021-06-25 06:15:56'),(8,3,'SampleApp','default','application','20161009155425-d3a0749c6e20bc15','2021-06-25 06:15:56','2021-06-25 06:15:56','2021-06-25 17:22:34'),(9,3,'SampleApp','default','application.xml','20191227212853-aeee1169c4a6ff5b','2021-06-25 06:15:59','2021-06-25 06:15:59','2021-06-25 06:15:59'),(10,4,'SampleApp','default','application','20161009155425-d3a0749c6e20bc15','2021-06-25 06:15:59','2021-06-25 06:15:59','2021-06-25 06:15:59'),(11,3,'SampleApp','default','application.json','20191227212956-0b3b1169c4a6ff5c','2021-06-25 06:15:59','2021-06-25 06:15:59','2021-06-25 06:15:59'),(12,3,'SampleApp','default','application.txt','20191227213038-a1471169c4a6ff5e','2021-06-25 06:15:59','2021-06-25 06:15:59','2021-06-25 06:15:59'),(13,2,'SampleApp','default','application.xml','20191227212853-aeee1169c4a6ff5b','2021-06-25 17:28:14','2021-06-25 17:28:14','2021-06-25 17:28:14'),(14,2,'SampleApp','default','application.yml','20191227213027-b2af1169c4a6ff5d','2021-06-25 17:28:28','2021-06-25 17:28:28','2021-06-27 12:06:17'),(15,5,'TestApp1','default','application','20210627203436-41101ed7973d8a3e','2021-06-27 12:35:10','2021-06-26 01:14:33','2021-07-03 05:41:56'),(16,6,'TestApp1','default','application','20210627023336-41101ed797164eaa','2021-06-26 18:33:37','2021-06-26 17:01:04','2021-06-27 02:50:05'),(17,6,'TestApp1','default','foo.yml','20210627105208-b2701ed79704ed24','2021-06-27 02:53:20','2021-06-27 02:53:20','2021-06-27 02:53:20'),(18,6,'TestApp1','default','foo1','20210627105548-47eb1ed79704ed25','2021-06-27 02:55:51','2021-06-27 02:55:51','2021-06-27 02:55:51'),(19,6,'TestApp1','default','foo2','20210627105616-47ec1ed79704ed26','2021-06-27 02:56:30','2021-06-27 02:56:30','2021-06-27 12:06:17'),(20,2,'SampleApp','default','application.json','20191227212956-0b3b1169c4a6ff5c','2021-06-27 12:14:31','2021-06-27 12:14:31','2021-07-03 05:41:56'),(21,5,'TestApp1','default','foo2','20210627203421-47ec1ed7973d8a3d','2021-06-28 06:05:22','2021-06-28 06:05:22','2021-07-03 02:55:32'),(22,5,'TestApp1','default','foo1','20210627203409-47eb1ed7973d8a3c','2021-06-28 06:05:22','2021-06-28 06:05:22','2021-07-03 02:55:32'),(23,7,'TestApp2','default','watcher','20210703151350-193d1ed797791be8','2021-07-03 07:13:51','2021-07-03 05:42:54','2021-07-03 07:13:52'),(24,7,'TestApp2','default','watcher2.json','20210703134319-51611ed797791be7','2021-07-03 05:43:23','2021-07-03 05:43:23','2021-07-03 05:43:23');
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
) ENGINE=InnoDB AUTO_INCREMENT=23 DEFAULT CHARSET=utf8mb4 COMMENT='配置项目';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Item`
--

LOCK TABLES `Item` WRITE;
/*!40000 ALTER TABLE `Item` DISABLE KEYS */;
INSERT INTO `Item` VALUES (1,1,'timeout','100','sample timeout配置',1,'\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,2,'content','<app>\n    <timeout>100</timeout>\n</app>',NULL,1,'\0','apollo','2019-12-27 13:28:52','apollo','2019-12-27 13:28:52'),(3,3,'content','{\"timeout\": 100}',NULL,1,'\0','apollo','2019-12-27 13:29:54','apollo','2019-12-27 13:29:54'),(4,4,'content','app:\n    id: 5\n    timeout: 100',NULL,1,'\0','apollo','2019-12-27 13:30:25','apollo','2019-12-27 13:30:25'),(5,5,'content','timeout is 100',NULL,1,'\0','apollo','2019-12-27 13:30:37','apollo','2019-12-27 13:30:37'),(6,6,'foo','bar',NULL,1,'','apollo','2021-06-26 01:13:59','apollo','2021-06-27 12:34:31'),(7,6,'foo1','bar1',NULL,2,'','apollo','2021-06-26 01:14:09','apollo','2021-06-27 12:34:33'),(8,6,'foo2','bar2=bar2=','',3,'','apollo','2021-06-26 01:14:57','apollo','2021-06-27 12:34:35'),(9,7,'content','foo:\n- 1\n- 2\n- 3',NULL,1,'\0','apollo','2021-06-27 02:52:04','apollo','2021-06-27 02:52:04'),(10,8,'a','1',NULL,1,'','apollo','2021-06-27 02:55:47','apollo','2021-06-27 12:33:58'),(11,9,'a','1',NULL,1,'','apollo','2021-06-27 02:56:15','apollo','2021-06-27 12:34:13'),(12,8,'foo1','bar1',NULL,1,'\0','apollo','2021-06-27 12:34:06','apollo','2021-06-27 12:34:06'),(13,9,'foo2','bar2',NULL,1,'\0','apollo','2021-06-27 12:34:19','apollo','2021-06-27 12:34:19'),(14,10,'timeout','100','',1,'','apollo','2021-06-28 08:22:07','apollo','2021-06-28 08:22:21'),(15,10,'timeout','3000',NULL,1,'','apollo','2021-06-28 08:28:19','apollo','2021-06-28 08:33:52'),(16,10,'timeout','3000',NULL,1,'','apollo','2021-06-28 08:33:55','apollo','2021-06-28 08:36:46'),(17,10,'connect_timeout','100','connect timeout',2,'','apollo','2021-06-28 08:33:55','apollo','2021-06-28 08:36:48'),(18,10,'timeout','3000',NULL,1,'','apollo','2021-06-28 08:36:51','apollo','2021-06-28 15:41:34'),(19,10,'connect_timeout','100','connect timeout',2,'','apollo','2021-06-28 08:36:51','apollo','2021-06-28 15:41:36'),(20,10,'timeout','3000',NULL,1,'\0','apollo','2021-06-28 15:41:41','apollo','2021-06-28 15:41:41'),(21,10,'connect_timeout','100','connect timeout',2,'\0','apollo','2021-06-28 15:41:41','apollo','2021-06-28 15:41:41'),(22,11,'a','1','a comment',1,'','apollo','2021-07-03 07:13:51','apollo','2021-07-03 07:14:19');
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
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COMMENT='命名空间';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Namespace`
--

LOCK TABLES `Namespace` WRITE;
/*!40000 ALTER TABLE `Namespace` DISABLE KEYS */;
INSERT INTO `Namespace` VALUES (1,'SampleApp','default','application','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'SampleApp','default','application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(3,'SampleApp','default','application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(4,'SampleApp','default','application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(5,'SampleApp','default','application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(6,'TestApp1','default','application','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(7,'TestApp1','default','foo.yml','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(8,'TestApp1','default','foo1','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(9,'TestApp1','default','foo2','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(10,'TestApp2','default','application','\0','apollo','2021-06-28 06:49:44','apollo','2021-06-28 06:49:44'),(11,'TestApp2','default','watcher','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(12,'TestApp2','default','watcher2.json','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54');
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
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COMMENT='发布';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Release`
--

LOCK TABLES `Release` WRITE;
/*!40000 ALTER TABLE `Release` DISABLE KEYS */;
INSERT INTO `Release` VALUES (1,'20161009155425-d3a0749c6e20bc15','20161009155424-release','Sample发布','SampleApp','default','application','{\"timeout\":\"100\"}','\0','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'20191227212853-aeee1169c4a6ff5b','20191227212852-release','','SampleApp','default','application.xml','{\"content\":\"\\u003capp\\u003e\\n    \\u003ctimeout\\u003e100\\u003c/timeout\\u003e\\n\\u003c/app\\u003e\"}','\0','\0','apollo','2019-12-27 13:28:54','apollo','2019-12-27 13:28:54'),(3,'20191227212956-0b3b1169c4a6ff5c','20191227212955-release','','SampleApp','default','application.json','{\"content\":\"{\\\"timeout\\\": 100}\"}','\0','\0','apollo','2019-12-27 13:29:56','apollo','2019-12-27 13:29:56'),(4,'20191227213027-b2af1169c4a6ff5d','20191227213026-release','','SampleApp','default','application.yml','{\"content\":\"app:\\n    id: 5\\n    timeout: 100\"}','\0','\0','apollo','2019-12-27 13:30:27','apollo','2019-12-27 13:30:27'),(5,'20191227213038-a1471169c4a6ff5e','20191227213037-release','','SampleApp','default','application.txt','{\"content\":\"timeout is 100\"}','\0','\0','apollo','2019-12-27 13:30:38','apollo','2019-12-27 13:30:38'),(6,'20210626091412-41101ed79747a76b','20210626091410-release','','TestApp1','default','application','{\"foo\":\"bar\",\"foo1\":\"bar1\"}','\0','\0','apollo','2021-06-26 01:14:12','apollo','2021-06-26 01:14:12'),(7,'20210626091501-41101ed79747a76c','20210626091500-release','','TestApp1','default','application','{\"foo\":\"bar\",\"foo1\":\"bar1\",\"foo2\":\"bar2\\u003dbar2\"}','\0','\0','apollo','2021-06-26 01:15:01','apollo','2021-06-26 01:15:01'),(8,'20210627023336-41101ed797164eaa','20210627023334-release','','TestApp1','default','application','{\"foo\":\"bar\",\"foo1\":\"bar1\",\"foo2\":\"bar2\\u003dbar2\\u003d\"}','\0','\0','apollo','2021-06-26 18:33:36','apollo','2021-06-26 18:33:36'),(9,'20210627105208-b2701ed79704ed24','20210627105207-release','','TestApp1','default','foo.yml','{\"content\":\"foo:\\n- 1\\n- 2\\n- 3\"}','\0','\0','apollo','2021-06-27 02:52:08','apollo','2021-06-27 02:52:08'),(10,'20210627105548-47eb1ed79704ed25','20210627105547-release','','TestApp1','default','foo1','{\"a\":\"1\"}','\0','\0','apollo','2021-06-27 02:55:48','apollo','2021-06-27 02:55:48'),(11,'20210627105616-47ec1ed79704ed26','20210627105615-release','','TestApp1','default','foo2','{\"a\":\"1\"}','\0','\0','apollo','2021-06-27 02:56:16','apollo','2021-06-27 02:56:16'),(12,'20210627203409-47eb1ed7973d8a3c','20210627203408-release','','TestApp1','default','foo1','{\"foo1\":\"bar1\"}','\0','\0','apollo','2021-06-27 12:34:09','apollo','2021-06-27 12:34:09'),(13,'20210627203421-47ec1ed7973d8a3d','20210627203420-release','','TestApp1','default','foo2','{\"foo2\":\"bar2\"}','\0','\0','apollo','2021-06-27 12:34:21','apollo','2021-06-27 12:34:21'),(14,'20210627203436-41101ed7973d8a3e','20210627203435-release','','TestApp1','default','application','{}','\0','\0','apollo','2021-06-27 12:34:37','apollo','2021-06-27 12:34:37'),(15,'20210628161215-44d11ed797c3240e','20210628161214-release','','TestApp2','default','application','{}','\0','\0','apollo','2021-06-28 08:12:16','apollo','2021-06-28 08:12:16'),(16,'20210628234405-44d11ed797b57c19','test-release','','TestApp2','default','application','{\"timeout\":\"3000\",\"connect_timeout\":\"100\"}','\0','\0','apollo','2021-06-28 15:44:05','apollo','2021-06-28 15:44:05'),(17,'20210703114911-193d1ed797791be6','20210703114910-release','','TestApp2','default','watcher','{}','\0','\0','apollo','2021-07-03 03:49:11','apollo','2021-07-03 03:49:11'),(18,'20210703134319-51611ed797791be7','20210703134318-release','','TestApp2','default','watcher2.json','{}','\0','\0','apollo','2021-07-03 05:43:20','apollo','2021-07-03 05:43:20'),(19,'20210703151350-193d1ed797791be8','release a','release a comment','TestApp2','default','watcher','{\"a\":\"1\"}','\0','\0','apollo','2021-07-03 07:13:51','apollo','2021-07-03 07:13:51'),(20,'20210703151421-193d1ed797791be9','20210703151420-release','','TestApp2','default','watcher','{}','\0','\0','apollo','2021-07-03 07:14:21','apollo','2021-07-03 07:14:21');
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
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COMMENT='发布历史';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ReleaseHistory`
--

LOCK TABLES `ReleaseHistory` WRITE;
/*!40000 ALTER TABLE `ReleaseHistory` DISABLE KEYS */;
INSERT INTO `ReleaseHistory` VALUES (1,'SampleApp','default','application','default',1,0,0,'{}','\0','apollo','2019-12-27 13:12:54','apollo','2019-12-27 13:12:54'),(2,'SampleApp','default','application.xml','default',2,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:28:54','apollo','2019-12-27 13:28:54'),(3,'SampleApp','default','application.json','default',3,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:29:56','apollo','2019-12-27 13:29:56'),(4,'SampleApp','default','application.yml','default',4,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:30:27','apollo','2019-12-27 13:30:27'),(5,'SampleApp','default','application.txt','default',5,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2019-12-27 13:30:38','apollo','2019-12-27 13:30:38'),(6,'TestApp1','default','application','default',6,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-26 01:14:12','apollo','2021-06-26 01:14:12'),(7,'TestApp1','default','application','default',7,6,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-26 01:15:01','apollo','2021-06-26 01:15:01'),(8,'TestApp1','default','application','default',8,7,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-26 18:33:36','apollo','2021-06-26 18:33:36'),(9,'TestApp1','default','foo.yml','default',9,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-27 02:52:08','apollo','2021-06-27 02:52:08'),(10,'TestApp1','default','foo1','default',10,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-27 02:55:48','apollo','2021-06-27 02:55:48'),(11,'TestApp1','default','foo2','default',11,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-27 02:56:16','apollo','2021-06-27 02:56:16'),(12,'TestApp1','default','foo1','default',12,10,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-27 12:34:09','apollo','2021-06-27 12:34:09'),(13,'TestApp1','default','foo2','default',13,11,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-27 12:34:21','apollo','2021-06-27 12:34:21'),(14,'TestApp1','default','application','default',14,8,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-27 12:34:37','apollo','2021-06-27 12:34:37'),(15,'TestApp2','default','application','default',15,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-28 08:12:16','apollo','2021-06-28 08:12:16'),(16,'TestApp2','default','application','default',16,15,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-06-28 15:44:05','apollo','2021-06-28 15:44:05'),(17,'TestApp2','default','watcher','default',17,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-07-03 03:49:11','apollo','2021-07-03 03:49:11'),(18,'TestApp2','default','watcher2.json','default',18,0,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-07-03 05:43:20','apollo','2021-07-03 05:43:20'),(19,'TestApp2','default','watcher','default',19,17,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-07-03 07:13:51','apollo','2021-07-03 07:13:51'),(20,'TestApp2','default','watcher','default',20,19,0,'{\"isEmergencyPublish\":false}','\0','apollo','2021-07-03 07:14:21','apollo','2021-07-03 07:14:21');
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
) ENGINE=InnoDB AUTO_INCREMENT=20 DEFAULT CHARSET=utf8mb4 COMMENT='发布消息';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ReleaseMessage`
--

LOCK TABLES `ReleaseMessage` WRITE;
/*!40000 ALTER TABLE `ReleaseMessage` DISABLE KEYS */;
INSERT INTO `ReleaseMessage` VALUES (1,'SampleApp+default+application.xml','2019-12-27 13:28:54'),(2,'SampleApp+default+application.json','2019-12-27 13:29:56'),(3,'SampleApp+default+application.yml','2019-12-27 13:30:27'),(4,'SampleApp+default+application.txt','2019-12-27 13:30:38'),(8,'TestApp1+default+foo.yml','2021-06-27 02:52:08'),(11,'TestApp1+default+foo1','2021-06-27 12:34:09'),(12,'TestApp1+default+foo2','2021-06-27 12:34:21'),(13,'TestApp1+default+application','2021-06-27 12:34:37'),(15,'TestApp2+default+application','2021-06-28 15:44:05'),(17,'TestApp2+default+watcher2.json','2021-07-03 05:43:20'),(19,'TestApp2+default+watcher','2021-07-03 07:14:21');
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
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COMMENT='应用表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `App`
--

LOCK TABLES `App` WRITE;
/*!40000 ALTER TABLE `App` DISABLE KEYS */;
INSERT INTO `App` VALUES (1,'SampleApp','Sample App','TEST1','样例部门1','apollo','apollo@acme.com','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'TestApp1','TestApp1','TEST1','样例部门1','apollo','apollo@acme.com','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(3,'TestApp2','TestApp2','TEST1','样例部门1','apollo','apollo@acme.com','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43');
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
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8mb4 COMMENT='应用namespace定义';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `AppNamespace`
--

LOCK TABLES `AppNamespace` WRITE;
/*!40000 ALTER TABLE `AppNamespace` DISABLE KEYS */;
INSERT INTO `AppNamespace` VALUES (1,'application','SampleApp','properties','\0','default app namespace','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'application.xml','SampleApp','xml','\0','','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(3,'application.json','SampleApp','json','\0','','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(4,'application.yml','SampleApp','yml','\0','','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(5,'application.txt','SampleApp','txt','\0','','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(6,'application','TestApp1','properties','\0','default app namespace','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(7,'foo.yml','TestApp1','yml','\0','','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(8,'foo1','TestApp1','properties','\0','','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(9,'foo2','TestApp1','properties','\0','','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(10,'application','TestApp2','properties','\0','default app namespace','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(11,'watcher','TestApp2','properties','\0','','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(12,'watcher2.json','TestApp2','json','\0','','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54');
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
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COMMENT='开放API消费者';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Consumer`
--

LOCK TABLES `Consumer` WRITE;
/*!40000 ALTER TABLE `Consumer` DISABLE KEYS */;
INSERT INTO `Consumer` VALUES (1,'1000','apollo','TEST1','样例部门1','apollo','apollo@acme.com','\0','apollo','2021-06-25 07:26:33','apollo','2021-06-25 07:26:33');
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
) ENGINE=InnoDB AUTO_INCREMENT=30 DEFAULT CHARSET=utf8mb4 COMMENT='consumer审计表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ConsumerAudit`
--

LOCK TABLES `ConsumerAudit` WRITE;
/*!40000 ALTER TABLE `ConsumerAudit` DISABLE KEYS */;
INSERT INTO `ConsumerAudit` VALUES (1,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:03:37','2021-06-28 08:03:37'),(2,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:04:01','2021-06-28 08:04:01'),(3,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:11:00','2021-06-28 08:11:00'),(4,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:11:35','2021-06-28 08:11:35'),(5,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:12:21','2021-06-28 08:12:21'),(6,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:16:40','2021-06-28 08:16:40'),(7,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:17:31','2021-06-28 08:17:31'),(8,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:20:15','2021-06-28 08:20:15'),(9,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:21:04','2021-06-28 08:21:04'),(10,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:22:07','2021-06-28 08:22:07'),(11,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:28:19','2021-06-28 08:28:19'),(12,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:33:09','2021-06-28 08:33:09'),(13,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:33:55','2021-06-28 08:33:55'),(14,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:33:55','2021-06-28 08:33:55'),(15,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:36:30','2021-06-28 08:36:30'),(16,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:36:51','2021-06-28 08:36:51'),(17,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:36:51','2021-06-28 08:36:51'),(18,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 08:36:51','2021-06-28 08:36:51'),(19,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 15:40:20','2021-06-28 15:40:20'),(20,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 15:41:41','2021-06-28 15:41:41'),(21,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 15:41:41','2021-06-28 15:41:41'),(22,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 15:41:41','2021-06-28 15:41:41'),(23,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/releases','POST','2021-06-28 15:41:41','2021-06-28 15:41:41'),(24,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/releases','POST','2021-06-28 15:42:41','2021-06-28 15:42:41'),(25,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/releases','POST','2021-06-28 15:44:05','2021-06-28 15:44:05'),(26,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 15:45:36','2021-06-28 15:45:36'),(27,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/application/items','POST','2021-06-28 15:45:57','2021-06-28 15:45:57'),(28,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/watcher/items','POST','2021-07-03 07:13:51','2021-07-03 07:13:51'),(29,1,'/openapi/v1/envs/DEV/apps/TestApp2/clusters/default/namespaces/watcher/releases','POST','2021-07-03 07:13:51','2021-07-03 07:13:51');
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
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COMMENT='consumer和role的绑定表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ConsumerRole`
--

LOCK TABLES `ConsumerRole` WRITE;
/*!40000 ALTER TABLE `ConsumerRole` DISABLE KEYS */;
INSERT INTO `ConsumerRole` VALUES (1,1,39,'\0','apollo','2021-06-28 08:04:50','apollo','2021-06-28 08:04:50');
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
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COMMENT='consumer token表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ConsumerToken`
--

LOCK TABLES `ConsumerToken` WRITE;
/*!40000 ALTER TABLE `ConsumerToken` DISABLE KEYS */;
INSERT INTO `ConsumerToken` VALUES (1,1,'391cc4053f8cce2e452a0e6db8925bbba503f434','2099-01-01 00:00:00','\0','apollo','2021-06-25 07:26:33','apollo','2021-06-25 07:26:33');
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
) ENGINE=InnoDB AUTO_INCREMENT=59 DEFAULT CHARSET=utf8mb4 COMMENT='permission表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Permission`
--

LOCK TABLES `Permission` WRITE;
/*!40000 ALTER TABLE `Permission` DISABLE KEYS */;
INSERT INTO `Permission` VALUES (1,'CreateCluster','SampleApp','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'CreateNamespace','SampleApp','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'AssignRole','SampleApp','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'ModifyNamespace','SampleApp+application','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(5,'ReleaseNamespace','SampleApp+application','\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(6,'CreateApplication','SystemRole','\0','apollo','2019-12-27 13:23:32','apollo','2019-12-27 13:23:32'),(7,'ModifyNamespace','SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(8,'ReleaseNamespace','SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(9,'ModifyNamespace','SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(10,'ReleaseNamespace','SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(11,'ModifyNamespace','SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(12,'ReleaseNamespace','SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(13,'ModifyNamespace','SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(14,'ReleaseNamespace','SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(15,'ModifyNamespace','SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(16,'ReleaseNamespace','SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(17,'ModifyNamespace','SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(18,'ReleaseNamespace','SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(19,'ModifyNamespace','SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(20,'ReleaseNamespace','SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(21,'ModifyNamespace','SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(22,'ReleaseNamespace','SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(23,'CreateNamespace','TestApp1','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(24,'AssignRole','TestApp1','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(25,'CreateCluster','TestApp1','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(26,'ManageAppMaster','TestApp1','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(27,'ModifyNamespace','TestApp1+application','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(28,'ReleaseNamespace','TestApp1+application','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(29,'ModifyNamespace','TestApp1+application+DEV','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(30,'ReleaseNamespace','TestApp1+application+DEV','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(31,'ModifyNamespace','TestApp1+foo.yml','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(32,'ReleaseNamespace','TestApp1+foo.yml','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(33,'ModifyNamespace','TestApp1+foo.yml+DEV','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(34,'ReleaseNamespace','TestApp1+foo.yml+DEV','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(35,'ModifyNamespace','TestApp1+foo1','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(36,'ReleaseNamespace','TestApp1+foo1','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(37,'ModifyNamespace','TestApp1+foo1+DEV','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(38,'ReleaseNamespace','TestApp1+foo1+DEV','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(39,'ModifyNamespace','TestApp1+foo2','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(40,'ReleaseNamespace','TestApp1+foo2','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(41,'ModifyNamespace','TestApp1+foo2+DEV','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(42,'ReleaseNamespace','TestApp1+foo2+DEV','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(43,'CreateNamespace','TestApp2','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(44,'AssignRole','TestApp2','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(45,'CreateCluster','TestApp2','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(46,'ManageAppMaster','TestApp2','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(47,'ModifyNamespace','TestApp2+application','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(48,'ReleaseNamespace','TestApp2+application','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(49,'ModifyNamespace','TestApp2+application+DEV','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(50,'ReleaseNamespace','TestApp2+application+DEV','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(51,'ModifyNamespace','TestApp2+watcher','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(52,'ReleaseNamespace','TestApp2+watcher','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(53,'ModifyNamespace','TestApp2+watcher+DEV','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(54,'ReleaseNamespace','TestApp2+watcher+DEV','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(55,'ModifyNamespace','TestApp2+watcher2.json','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(56,'ReleaseNamespace','TestApp2+watcher2.json','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(57,'ModifyNamespace','TestApp2+watcher2.json+DEV','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(58,'ReleaseNamespace','TestApp2+watcher2.json+DEV','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54');
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
) ENGINE=InnoDB AUTO_INCREMENT=53 DEFAULT CHARSET=utf8mb4 COMMENT='角色表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Role`
--

LOCK TABLES `Role` WRITE;
/*!40000 ALTER TABLE `Role` DISABLE KEYS */;
INSERT INTO `Role` VALUES (1,'Master+SampleApp','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'ModifyNamespace+SampleApp+application','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'ReleaseNamespace+SampleApp+application','\0','default','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'CreateApplication+SystemRole','\0','apollo','2019-12-27 13:23:32','apollo','2019-12-27 13:23:32'),(5,'ModifyNamespace+SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(6,'ReleaseNamespace+SampleApp+application.xml','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(7,'ModifyNamespace+SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(8,'ReleaseNamespace+SampleApp+application.xml+DEV','\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(9,'ModifyNamespace+SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(10,'ReleaseNamespace+SampleApp+application.json','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(11,'ModifyNamespace+SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(12,'ReleaseNamespace+SampleApp+application.json+DEV','\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(13,'ModifyNamespace+SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(14,'ReleaseNamespace+SampleApp+application.yml','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(15,'ModifyNamespace+SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(16,'ReleaseNamespace+SampleApp+application.yml+DEV','\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(17,'ModifyNamespace+SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(18,'ReleaseNamespace+SampleApp+application.txt','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(19,'ModifyNamespace+SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(20,'ReleaseNamespace+SampleApp+application.txt+DEV','\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(21,'Master+TestApp1','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(22,'ManageAppMaster+TestApp1','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(23,'ModifyNamespace+TestApp1+application','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(24,'ReleaseNamespace+TestApp1+application','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(25,'ModifyNamespace+TestApp1+application+DEV','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(26,'ReleaseNamespace+TestApp1+application+DEV','\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(27,'ModifyNamespace+TestApp1+foo.yml','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(28,'ReleaseNamespace+TestApp1+foo.yml','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(29,'ModifyNamespace+TestApp1+foo.yml+DEV','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(30,'ReleaseNamespace+TestApp1+foo.yml+DEV','\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(31,'ModifyNamespace+TestApp1+foo1','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(32,'ReleaseNamespace+TestApp1+foo1','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(33,'ModifyNamespace+TestApp1+foo1+DEV','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(34,'ReleaseNamespace+TestApp1+foo1+DEV','\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(35,'ModifyNamespace+TestApp1+foo2','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(36,'ReleaseNamespace+TestApp1+foo2','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(37,'ModifyNamespace+TestApp1+foo2+DEV','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(38,'ReleaseNamespace+TestApp1+foo2+DEV','\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(39,'Master+TestApp2','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(40,'ManageAppMaster+TestApp2','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(41,'ModifyNamespace+TestApp2+application','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(42,'ReleaseNamespace+TestApp2+application','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(43,'ModifyNamespace+TestApp2+application+DEV','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(44,'ReleaseNamespace+TestApp2+application+DEV','\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(45,'ModifyNamespace+TestApp2+watcher','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(46,'ReleaseNamespace+TestApp2+watcher','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(47,'ModifyNamespace+TestApp2+watcher+DEV','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(48,'ReleaseNamespace+TestApp2+watcher+DEV','\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(49,'ModifyNamespace+TestApp2+watcher2.json','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(50,'ReleaseNamespace+TestApp2+watcher2.json','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(51,'ModifyNamespace+TestApp2+watcher2.json+DEV','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(52,'ReleaseNamespace+TestApp2+watcher2.json+DEV','\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54');
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
) ENGINE=InnoDB AUTO_INCREMENT=59 DEFAULT CHARSET=utf8mb4 COMMENT='角色和权限的绑定表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `RolePermission`
--

LOCK TABLES `RolePermission` WRITE;
/*!40000 ALTER TABLE `RolePermission` DISABLE KEYS */;
INSERT INTO `RolePermission` VALUES (1,1,1,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,1,2,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,1,3,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,2,4,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(5,3,5,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(6,4,6,'\0','apollo','2019-12-27 13:23:32','apollo','2019-12-27 13:23:32'),(7,5,7,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(8,6,8,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(9,7,9,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(10,8,10,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(11,9,11,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(12,10,12,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(13,11,13,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(14,12,14,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(15,13,15,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(16,14,16,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(17,15,17,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(18,16,18,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(19,17,19,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(20,18,20,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(21,19,21,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(22,20,22,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(23,21,23,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(24,21,24,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(25,21,25,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(26,22,26,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(27,23,27,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(28,24,28,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(29,25,29,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(30,26,30,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(31,27,31,'\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(32,28,32,'\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(33,29,33,'\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(34,30,34,'\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(35,31,35,'\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(36,32,36,'\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(37,33,37,'\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(38,34,38,'\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(39,35,39,'\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(40,36,40,'\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(41,37,41,'\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(42,38,42,'\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(43,39,43,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(44,39,44,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(45,39,45,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(46,40,46,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(47,41,47,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(48,42,48,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(49,43,49,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(50,44,50,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(51,45,51,'\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(52,46,52,'\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(53,47,53,'\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(54,48,54,'\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(55,49,55,'\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(56,50,56,'\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(57,51,57,'\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(58,52,58,'\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54');
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
) ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8mb4 COMMENT='用户和role的绑定表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `UserRole`
--

LOCK TABLES `UserRole` WRITE;
/*!40000 ALTER TABLE `UserRole` DISABLE KEYS */;
INSERT INTO `UserRole` VALUES (1,'apollo',1,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(2,'apollo',2,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(3,'apollo',3,'\0','','2019-12-27 13:12:54','','2019-12-27 13:12:54'),(4,'apollo',5,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(5,'apollo',6,'\0','apollo','2019-12-27 13:27:14','apollo','2019-12-27 13:27:14'),(6,'apollo',9,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(7,'apollo',10,'\0','apollo','2019-12-27 13:27:26','apollo','2019-12-27 13:27:26'),(8,'apollo',13,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(9,'apollo',14,'\0','apollo','2019-12-27 13:28:03','apollo','2019-12-27 13:28:03'),(10,'apollo',17,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(11,'apollo',18,'\0','apollo','2019-12-27 13:28:12','apollo','2019-12-27 13:28:12'),(12,'apollo',21,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(13,'apollo',23,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(14,'apollo',24,'\0','apollo','2021-06-26 01:13:44','apollo','2021-06-26 01:13:44'),(15,'apollo',27,'\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(16,'apollo',28,'\0','apollo','2021-06-27 02:50:57','apollo','2021-06-27 02:50:57'),(17,'apollo',31,'\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(18,'apollo',32,'\0','apollo','2021-06-27 02:54:19','apollo','2021-06-27 02:54:19'),(19,'apollo',35,'\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(20,'apollo',36,'\0','apollo','2021-06-27 02:56:05','apollo','2021-06-27 02:56:05'),(21,'apollo',39,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(22,'apollo',41,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(23,'apollo',42,'\0','apollo','2021-06-28 06:49:43','apollo','2021-06-28 06:49:43'),(24,'apollo',45,'\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(25,'apollo',46,'\0','apollo','2021-07-03 03:49:06','apollo','2021-07-03 03:49:06'),(26,'apollo',49,'\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54'),(27,'apollo',50,'\0','apollo','2021-07-03 05:34:54','apollo','2021-07-03 05:34:54');
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

-- Dump completed on 2021-07-03 16:03:36
