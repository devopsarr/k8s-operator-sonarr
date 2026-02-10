# API Reference

Packages:

- [devopsarr.io/v1alpha1](#devopsarriov1alpha1)

# devopsarr.io/v1alpha1

Resource Types:

- [SonarrAutoTag](#sonarrautotag)

- [SonarrCustomFormat](#sonarrcustomformat)

- [SonarrDelayProfile](#sonarrdelayprofile)

- [SonarrDownloadClient](#sonarrdownloadclient)

- [SonarrDownloadClientConfig](#sonarrdownloadclientconfig)

- [SonarrImportList](#sonarrimportlist)

- [SonarrIndexer](#sonarrindexer)

- [SonarrIndexerConfig](#sonarrindexerconfig)

- [SonarrLanguageProfile](#sonarrlanguageprofile)

- [SonarrMediaManagementConfig](#sonarrmediamanagementconfig)

- [SonarrMetadata](#sonarrmetadata)

- [SonarrNamingConfig](#sonarrnamingconfig)

- [SonarrNotification](#sonarrnotification)

- [SonarrQualityDefinition](#sonarrqualitydefinition)

- [SonarrQualityProfile](#sonarrqualityprofile)

- [SonarrRootFolder](#sonarrrootfolder)

- [SonarrSeries](#sonarrseries)

- [Sonarr](#sonarr)

- [SonarrTag](#sonarrtag)




## SonarrAutoTag
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrAutoTagSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrAutoTag</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrautotagspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrAutoTag represents an auto-tagging rule configuration in Sonarr
Auto-tagging automatically applies tags to series based on conditions<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrautotagstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrAutoTag.spec
<sup><sup>[↩ Parent](#sonarrautotag)</sup></sup>



SonarrAutoTag represents an auto-tagging rule configuration in Sonarr
Auto-tagging automatically applies tags to series based on conditions

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Auto-tag rule name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrautotagspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>removeTagsAutomatically</b></td>
        <td>boolean</td>
        <td>
          Remove tags automatically when conditions no longer match<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrautotagspecspecificationsindex">specifications</a></b></td>
        <td>[]object</td>
        <td>
          Specifications (conditions) for this auto-tag rule<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags to apply when conditions match<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrAutoTag.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrautotagspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrAutoTag.spec.specifications[index]
<sup><sup>[↩ Parent](#sonarrautotagspec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>implementation</b></td>
        <td>enum</td>
        <td>
          Specification type/implementation<br/>
          <br/>
            <i>Enum</i>: rootFolderSpecification, genreSpecification, yearSpecification, seriesTypeSpecification, qualityProfileSpecification, networkSpecification, originalLanguageSpecification, tagSpecification<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Specification name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrautotagspecspecificationsindexfields">fields</a></b></td>
        <td>object</td>
        <td>
          Fields/values for this specification<br/>
          <br/>
            <i>Default</i>: map[max:<nil> min:<nil> value:<nil>]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>negate</b></td>
        <td>boolean</td>
        <td>
          Negate this condition<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>required</b></td>
        <td>boolean</td>
        <td>
          This condition is required<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrAutoTag.spec.specifications[index].fields
<sup><sup>[↩ Parent](#sonarrautotagspecspecificationsindex)</sup></sup>



Fields/values for this specification

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>max</b></td>
        <td>integer</td>
        <td>
          Maximum value (for year specifications)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>min</b></td>
        <td>integer</td>
        <td>
          Minimum value (for year specifications)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>value</b></td>
        <td>string</td>
        <td>
          Value for the specification (path, genre, network, etc.)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrAutoTag.status
<sup><sup>[↩ Parent](#sonarrautotag)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrautotagstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Auto Tag ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrAutoTag.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrautotagstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrCustomFormat
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrCustomFormatSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrCustomFormat</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrcustomformatspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrCustomFormat represents a custom format configuration in Sonarr
Custom formats are used to score releases based on various criteria<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrcustomformatstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrCustomFormat.spec
<sup><sup>[↩ Parent](#sonarrcustomformat)</sup></sup>



SonarrCustomFormat represents a custom format configuration in Sonarr
Custom formats are used to score releases based on various criteria

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Custom format name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrcustomformatspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>includeCustomFormatWhenRenaming</b></td>
        <td>boolean</td>
        <td>
          Include custom format name when renaming files<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrcustomformatspecspecificationsindex">specifications</a></b></td>
        <td>[]object</td>
        <td>
          Specifications (conditions) for this custom format<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrCustomFormat.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrcustomformatspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrCustomFormat.spec.specifications[index]
<sup><sup>[↩ Parent](#sonarrcustomformatspec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>implementation</b></td>
        <td>enum</td>
        <td>
          Specification type/implementation<br/>
          <br/>
            <i>Enum</i>: releaseTitleSpecification, sourceSpecification, resolutionSpecification, qualityModifierSpecification, sizeSpecification, indexerFlagSpecification, languageSpecification, releaseGroupSpecification, editionSpecification<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Specification name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrcustomformatspecspecificationsindexfields">fields</a></b></td>
        <td>object</td>
        <td>
          Fields/values for this specification<br/>
          <br/>
            <i>Default</i>: map[max:<nil> min:<nil> value:<nil>]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>negate</b></td>
        <td>boolean</td>
        <td>
          Negate this condition<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>required</b></td>
        <td>boolean</td>
        <td>
          This condition is required<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrCustomFormat.spec.specifications[index].fields
<sup><sup>[↩ Parent](#sonarrcustomformatspecspecificationsindex)</sup></sup>



Fields/values for this specification

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>max</b></td>
        <td>number</td>
        <td>
          Maximum value (for size specifications)<br/>
          <br/>
            <i>Format</i>: double<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>min</b></td>
        <td>number</td>
        <td>
          Minimum value (for size specifications)<br/>
          <br/>
            <i>Format</i>: double<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>value</b></td>
        <td>string</td>
        <td>
          Value for the specification (regex pattern, source type, etc.)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrCustomFormat.status
<sup><sup>[↩ Parent](#sonarrcustomformat)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrcustomformatstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Custom Format ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrCustomFormat.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrcustomformatstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrDelayProfile
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrDelayProfileSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrDelayProfile</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrdelayprofilespec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrDelayProfile represents a delay profile configuration in Sonarr
Delay profiles control how long Sonarr waits before grabbing a release<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrdelayprofilestatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDelayProfile.spec
<sup><sup>[↩ Parent](#sonarrdelayprofile)</sup></sup>



SonarrDelayProfile represents a delay profile configuration in Sonarr
Delay profiles control how long Sonarr waits before grabbing a release

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrdelayprofilespecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>bypassIfAboveCustomFormatScore</b></td>
        <td>boolean</td>
        <td>
          Bypass delay if above custom format score<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>bypassIfHighestQuality</b></td>
        <td>boolean</td>
        <td>
          Bypass delay if highest quality<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableTorrent</b></td>
        <td>boolean</td>
        <td>
          Enable Torrent downloads<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableUsenet</b></td>
        <td>boolean</td>
        <td>
          Enable Usenet downloads<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>minimumCustomFormatScore</b></td>
        <td>integer</td>
        <td>
          Minimum custom format score to bypass delay<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>order</b></td>
        <td>integer</td>
        <td>
          Order of this profile (lower = higher priority)<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>preferredProtocol</b></td>
        <td>enum</td>
        <td>
          Preferred download protocol<br/>
          <br/>
            <i>Enum</i>: usenet, torrent<br/>
            <i>Default</i>: usenet<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags to apply this delay profile to<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>torrentDelay</b></td>
        <td>integer</td>
        <td>
          Delay for Torrents in minutes<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>usenetDelay</b></td>
        <td>integer</td>
        <td>
          Delay for Usenet in minutes<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDelayProfile.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrdelayprofilespec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDelayProfile.status
<sup><sup>[↩ Parent](#sonarrdelayprofile)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrdelayprofilestatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Delay Profile ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDelayProfile.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrdelayprofilestatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrDownloadClient
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrDownloadClientSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrDownloadClient</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrDownloadClient represents a download client configuration in Sonarr
Download clients are used to download releases (qBittorrent, Transmission, SABnzbd, etc.)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.spec
<sup><sup>[↩ Parent](#sonarrdownloadclient)</sup></sup>



SonarrDownloadClient represents a download client configuration in Sonarr
Download clients are used to download releases (qBittorrent, Transmission, SABnzbd, etc.)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrdownloadclientspecconfig">config</a></b></td>
        <td>object</td>
        <td>
          Download client configuration<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>downloadClientType</b></td>
        <td>enum</td>
        <td>
          Download client type<br/>
          <br/>
            <i>Enum</i>: Aria2, Deluge, Flood, Hadouken, Nzbget, Nzbvortex, Pneumatic, QBittorrent, RTorrent, Sabnzbd, TorrentBlackhole, TorrentDownloadStation, Transmission, UsenetBlackhole, UsenetDownloadStation, UTorrent, Vuze<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Download client name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>enable</b></td>
        <td>boolean</td>
        <td>
          Enable this download client<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>priority</b></td>
        <td>integer</td>
        <td>
          Priority for this download client<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 1<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>removeCompletedDownloads</b></td>
        <td>boolean</td>
        <td>
          Remove completed downloads<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>removeFailedDownloads</b></td>
        <td>boolean</td>
        <td>
          Remove failed downloads<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags for this download client<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.spec.config
<sup><sup>[↩ Parent](#sonarrdownloadclientspec)</sup></sup>



Download client configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>addPaused</b></td>
        <td>boolean</td>
        <td>
          Add paused<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientspecconfigapikeysecretref">apiKeySecretRef</a></b></td>
        <td>object</td>
        <td>
          API key from secret (for some clients)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>firstAndLast</b></td>
        <td>boolean</td>
        <td>
          First and last (for qBittorrent)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>host</b></td>
        <td>string</td>
        <td>
          Host address<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>initialState</b></td>
        <td>integer</td>
        <td>
          Initial state (for qBittorrent: 0 = Start, 1 = ForceStart, 2 = Pause)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>nzbFolder</b></td>
        <td>string</td>
        <td>
          NZB folder (for blackhole)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>olderTvPriority</b></td>
        <td>integer</td>
        <td>
          Older TV priority (0 = Last, 1 = First)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientspecconfigpasswordsecretref">passwordSecretRef</a></b></td>
        <td>object</td>
        <td>
          Password from secret<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>port</b></td>
        <td>integer</td>
        <td>
          Port number<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>recentTvPriority</b></td>
        <td>integer</td>
        <td>
          Recent TV priority (0 = Last, 1 = First)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>rpcPath</b></td>
        <td>string</td>
        <td>
          RPC path (for Aria2)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>saveMagnetFiles</b></td>
        <td>boolean</td>
        <td>
          Save magnet files (for blackhole)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientspecconfigsecrettokensecretref">secretTokenSecretRef</a></b></td>
        <td>object</td>
        <td>
          Secret token (for Aria2)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>sequentialOrder</b></td>
        <td>boolean</td>
        <td>
          Sequential order (for qBittorrent)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>strmFolder</b></td>
        <td>string</td>
        <td>
          Strm folder (for pneumatic)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>torrentFolder</b></td>
        <td>string</td>
        <td>
          Torrent folder (for blackhole)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tvCategory</b></td>
        <td>string</td>
        <td>
          TV category<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tvDirectory</b></td>
        <td>string</td>
        <td>
          TV directory<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>urlBase</b></td>
        <td>string</td>
        <td>
          URL base path<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>useSsl</b></td>
        <td>boolean</td>
        <td>
          Use SSL<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>username</b></td>
        <td>string</td>
        <td>
          Username<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>watchFolder</b></td>
        <td>string</td>
        <td>
          Watch folder (for blackhole)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.spec.config.apiKeySecretRef
<sup><sup>[↩ Parent](#sonarrdownloadclientspecconfig)</sup></sup>



API key from secret (for some clients)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.spec.config.passwordSecretRef
<sup><sup>[↩ Parent](#sonarrdownloadclientspecconfig)</sup></sup>



Password from secret

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.spec.config.secretTokenSecretRef
<sup><sup>[↩ Parent](#sonarrdownloadclientspecconfig)</sup></sup>



Secret token (for Aria2)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrdownloadclientspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.status
<sup><sup>[↩ Parent](#sonarrdownloadclient)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrdownloadclientstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Download Client ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClient.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrdownloadclientstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrDownloadClientConfig
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrDownloadClientConfigSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrDownloadClientConfig</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientconfigspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrDownloadClientConfig configures global download client settings for a Sonarr instance.
Only one SonarrDownloadClientConfig per Sonarr instance is allowed.
Note: This is different from SonarrDownloadClient which configures individual download clients.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrdownloadclientconfigstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClientConfig.spec
<sup><sup>[↩ Parent](#sonarrdownloadclientconfig)</sup></sup>



SonarrDownloadClientConfig configures global download client settings for a Sonarr instance.
Only one SonarrDownloadClientConfig per Sonarr instance is allowed.
Note: This is different from SonarrDownloadClient which configures individual download clients.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrdownloadclientconfigspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the Sonarr instance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>autoRedownloadFailed</b></td>
        <td>boolean</td>
        <td>
          Automatically redownload failed releases<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>autoRedownloadFailedFromInteractiveSearch</b></td>
        <td>boolean</td>
        <td>
          Automatically redownload failed releases from interactive search<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>downloadClientWorkingFolders</b></td>
        <td>string</td>
        <td>
          Working folders for download client (container path mapping)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableCompletedDownloadHandling</b></td>
        <td>boolean</td>
        <td>
          Enable completed download handling<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClientConfig.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrdownloadclientconfigspec)</sup></sup>



Reference to the Sonarr instance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClientConfig.status
<sup><sup>[↩ Parent](#sonarrdownloadclientconfig)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrdownloadclientconfigstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrDownloadClientConfig.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrdownloadclientconfigstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrImportList
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrImportListSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrImportList</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrimportlistspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrImportList represents an import list configuration in Sonarr
Import lists automatically add series from external sources (Trakt, Plex, etc.)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrimportliststatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrImportList.spec
<sup><sup>[↩ Parent](#sonarrimportlist)</sup></sup>



SonarrImportList represents an import list configuration in Sonarr
Import lists automatically add series from external sources (Trakt, Plex, etc.)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>listType</b></td>
        <td>enum</td>
        <td>
          Import list type/implementation<br/>
          <br/>
            <i>Enum</i>: sonarrImport, traktListImport, traktUserImport, traktPopularImport, plexImport, imdbListImport, customImport, simklImport, aniListImport, myAnimeListImport<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Import list name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>qualityProfileId</b></td>
        <td>integer</td>
        <td>
          Quality profile ID to use<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>rootFolderPath</b></td>
        <td>string</td>
        <td>
          Root folder path for imported series<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrimportlistspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrimportlistspecconfig">config</a></b></td>
        <td>object</td>
        <td>
          Import list configuration<br/>
          <br/>
            <i>Default</i>: map[accessToken:<nil> apiKey:<nil> authUser:<nil> baseUrl:<nil> languageProfileId:<nil> listId:<nil> listname:<nil> profileIds:[] tagIds:[] traktListType:<nil> username:<nil>]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableAutomaticAdd</b></td>
        <td>boolean</td>
        <td>
          Enable automatic add<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>listOrder</b></td>
        <td>integer</td>
        <td>
          List order<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>monitorNewItems</b></td>
        <td>enum</td>
        <td>
          Monitor new items<br/>
          <br/>
            <i>Enum</i>: all, none<br/>
            <i>Default</i>: all<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>searchForMissingEpisodes</b></td>
        <td>boolean</td>
        <td>
          Search for missing episodes when adding<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seasonFolder</b></td>
        <td>boolean</td>
        <td>
          Use season folders<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seriesType</b></td>
        <td>enum</td>
        <td>
          Series type<br/>
          <br/>
            <i>Enum</i>: standard, daily, anime<br/>
            <i>Default</i>: standard<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>shouldMonitor</b></td>
        <td>enum</td>
        <td>
          Monitor type for imported series<br/>
          <br/>
            <i>Enum</i>: all, future, missing, existing, firstSeason, latestSeason, pilot, monitorSpecials, unmonitorSpecials, none<br/>
            <i>Default</i>: all<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags for imported series<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrImportList.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrimportlistspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrImportList.spec.config
<sup><sup>[↩ Parent](#sonarrimportlistspec)</sup></sup>



Import list configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>accessToken</b></td>
        <td>string</td>
        <td>
          Access token (for Trakt/Plex)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>apiKey</b></td>
        <td>string</td>
        <td>
          API key (for Sonarr import)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>authUser</b></td>
        <td>string</td>
        <td>
          Auth user (for Trakt)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>baseUrl</b></td>
        <td>string</td>
        <td>
          Base URL (for Sonarr import)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>languageProfileId</b></td>
        <td>integer</td>
        <td>
          Language profile ID (deprecated in v4)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>listId</b></td>
        <td>string</td>
        <td>
          List ID<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>listname</b></td>
        <td>string</td>
        <td>
          List name/ID<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>profileIds</b></td>
        <td>[]integer</td>
        <td>
          Profile IDs (for Sonarr import)<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tagIds</b></td>
        <td>[]integer</td>
        <td>
          Tag IDs (for Sonarr import)<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>traktListType</b></td>
        <td>integer</td>
        <td>
          Trakt list type<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>username</b></td>
        <td>string</td>
        <td>
          Username (for various services)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrImportList.status
<sup><sup>[↩ Parent](#sonarrimportlist)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrimportliststatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Import List ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrImportList.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrimportliststatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrIndexer
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrIndexerSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrIndexer</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrindexerspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrIndexer represents an indexer configuration in Sonarr
Indexers are sources for finding releases (Newznab, Torznab, etc.)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrindexerstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexer.spec
<sup><sup>[↩ Parent](#sonarrindexer)</sup></sup>



SonarrIndexer represents an indexer configuration in Sonarr
Indexers are sources for finding releases (Newznab, Torznab, etc.)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrindexerspecconfig">config</a></b></td>
        <td>object</td>
        <td>
          Indexer-specific configuration<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>indexerType</b></td>
        <td>enum</td>
        <td>
          Indexer type (Newznab, Torznab, etc.)<br/>
          <br/>
            <i>Enum</i>: newznab, torznab, fanzub, broadcasthenet, filelist, hdbits, iptorrents, nyaa, torrentrss, torrentleech<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Indexer name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrindexerspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>downloadClientId</b></td>
        <td>integer</td>
        <td>
          Download client ID to use<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableAutomaticSearch</b></td>
        <td>boolean</td>
        <td>
          Enable automatic search<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableInteractiveSearch</b></td>
        <td>boolean</td>
        <td>
          Enable interactive search<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableRss</b></td>
        <td>boolean</td>
        <td>
          Enable RSS feeds<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>priority</b></td>
        <td>integer</td>
        <td>
          Priority for this indexer<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 25<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags for this indexer<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexer.spec.config
<sup><sup>[↩ Parent](#sonarrindexerspec)</sup></sup>



Indexer-specific configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>additionalParameters</b></td>
        <td>string</td>
        <td>
          Additional parameters<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>animeCategories</b></td>
        <td>[]integer</td>
        <td>
          Anime categories<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>animeStandardFormatSearch</b></td>
        <td>boolean</td>
        <td>
          Search anime in standard format<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>apiKey</b></td>
        <td>string</td>
        <td>
          API key (can reference a secret)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrindexerspecconfigapikeysecretref">apiKeySecretRef</a></b></td>
        <td>object</td>
        <td>
          API key from secret reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>apiPath</b></td>
        <td>string</td>
        <td>
          API path (default: /api)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>baseUrl</b></td>
        <td>string</td>
        <td>
          Base URL for the indexer<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>categories</b></td>
        <td>[]integer</td>
        <td>
          Categories to search<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>cookie</b></td>
        <td>string</td>
        <td>
          Cookie (for some indexers)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>minimumSeeders</b></td>
        <td>integer</td>
        <td>
          Minimum seeders (for torrent indexers)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>passkey</b></td>
        <td>string</td>
        <td>
          Passkey (for some indexers)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrindexerspecconfigpasswordsecretref">passwordSecretRef</a></b></td>
        <td>object</td>
        <td>
          Password secret reference (for some indexers)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seedRatio</b></td>
        <td>number</td>
        <td>
          Seed ratio (for torrent indexers)<br/>
          <br/>
            <i>Format</i>: double<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seedTime</b></td>
        <td>integer</td>
        <td>
          Seed time (for torrent indexers)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>username</b></td>
        <td>string</td>
        <td>
          Username (for some indexers)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexer.spec.config.apiKeySecretRef
<sup><sup>[↩ Parent](#sonarrindexerspecconfig)</sup></sup>



API key from secret reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrIndexer.spec.config.passwordSecretRef
<sup><sup>[↩ Parent](#sonarrindexerspecconfig)</sup></sup>



Password secret reference (for some indexers)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrIndexer.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrindexerspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexer.status
<sup><sup>[↩ Parent](#sonarrindexer)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrindexerstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Indexer ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexer.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrindexerstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrIndexerConfig
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrIndexerConfigSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrIndexerConfig</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrindexerconfigspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrIndexerConfig configures global indexer settings for a Sonarr instance.
Only one SonarrIndexerConfig per Sonarr instance is allowed.
Note: This is different from SonarrIndexer which configures individual indexers.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrindexerconfigstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexerConfig.spec
<sup><sup>[↩ Parent](#sonarrindexerconfig)</sup></sup>



SonarrIndexerConfig configures global indexer settings for a Sonarr instance.
Only one SonarrIndexerConfig per Sonarr instance is allowed.
Note: This is different from SonarrIndexer which configures individual indexers.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrindexerconfigspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the Sonarr instance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>maximumSize</b></td>
        <td>integer</td>
        <td>
          Maximum release size in MB (0 = unlimited)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>minimumAge</b></td>
        <td>integer</td>
        <td>
          Minimum age in minutes before downloading (usenet)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>retention</b></td>
        <td>integer</td>
        <td>
          Retention in days (0 = unlimited)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>rssSyncInterval</b></td>
        <td>integer</td>
        <td>
          RSS sync interval in minutes (0 = disabled, minimum 10)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexerConfig.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrindexerconfigspec)</sup></sup>



Reference to the Sonarr instance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexerConfig.status
<sup><sup>[↩ Parent](#sonarrindexerconfig)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrindexerconfigstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrIndexerConfig.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrindexerconfigstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrLanguageProfile
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrLanguageProfileSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrLanguageProfile</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrlanguageprofilespec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrLanguageProfile represents a language profile configuration in Sonarr
Language profiles define preferred languages for downloading series
Note: Deprecated in Sonarr v4, replaced by per-series language selection<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrlanguageprofilestatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrLanguageProfile.spec
<sup><sup>[↩ Parent](#sonarrlanguageprofile)</sup></sup>



SonarrLanguageProfile represents a language profile configuration in Sonarr
Language profiles define preferred languages for downloading series
Note: Deprecated in Sonarr v4, replaced by per-series language selection

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>cutoffLanguage</b></td>
        <td>enum</td>
        <td>
          Cutoff language - stop upgrading when this language is reached<br/>
          <br/>
            <i>Enum</i>: Unknown, English, French, Spanish, German, Italian, Danish, Dutch, Japanese, Icelandic, Chinese, Russian, Polish, Vietnamese, Swedish, Norwegian, Finnish, Turkish, Portuguese, Flemish, Greek, Korean, Hungarian, Hebrew, Lithuanian, Czech, Hindi, Romanian, Thai, Bulgarian, PortugueseBrazil, Arabic, Ukrainian, Persian, Bengali, Slovak, Latvian, SpanishLatino, Catalan, Croatian, Serbian, Bosnian, Estonian, Tamil, Indonesian, Telugu, Macedonian, Slovenian, Malay, Original, Any<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrlanguageprofilespeclanguagesindex">languages</a></b></td>
        <td>[]object</td>
        <td>
          Ordered list of languages (first = highest priority)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Language profile name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrlanguageprofilespecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>upgradeAllowed</b></td>
        <td>boolean</td>
        <td>
          Allow upgrades to better quality languages<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrLanguageProfile.spec.languages[index]
<sup><sup>[↩ Parent](#sonarrlanguageprofilespec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>language</b></td>
        <td>enum</td>
        <td>
          Language<br/>
          <br/>
            <i>Enum</i>: Unknown, English, French, Spanish, German, Italian, Danish, Dutch, Japanese, Icelandic, Chinese, Russian, Polish, Vietnamese, Swedish, Norwegian, Finnish, Turkish, Portuguese, Flemish, Greek, Korean, Hungarian, Hebrew, Lithuanian, Czech, Hindi, Romanian, Thai, Bulgarian, PortugueseBrazil, Arabic, Ukrainian, Persian, Bengali, Slovak, Latvian, SpanishLatino, Catalan, Croatian, Serbian, Bosnian, Estonian, Tamil, Indonesian, Telugu, Macedonian, Slovenian, Malay, Original, Any<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>allowed</b></td>
        <td>boolean</td>
        <td>
          Whether this language is allowed<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrLanguageProfile.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrlanguageprofilespec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrLanguageProfile.status
<sup><sup>[↩ Parent](#sonarrlanguageprofile)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrlanguageprofilestatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Language Profile ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrLanguageProfile.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrlanguageprofilestatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrMediaManagementConfig
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrMediaManagementConfigSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrMediaManagementConfig</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrmediamanagementconfigspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrMediaManagementConfig configures media management settings for a Sonarr instance.
Only one SonarrMediaManagementConfig per Sonarr instance is allowed.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrmediamanagementconfigstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMediaManagementConfig.spec
<sup><sup>[↩ Parent](#sonarrmediamanagementconfig)</sup></sup>



SonarrMediaManagementConfig configures media management settings for a Sonarr instance.
Only one SonarrMediaManagementConfig per Sonarr instance is allowed.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrmediamanagementconfigspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the Sonarr instance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>autoUnmonitorPreviouslyDownloadedEpisodes</b></td>
        <td>boolean</td>
        <td>
          Auto unmonitor previously downloaded episodes when marked as deleted<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>chmodFolder</b></td>
        <td>string</td>
        <td>
          chmod folder permissions (e.g., "755")<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>chownGroup</b></td>
        <td>string</td>
        <td>
          chown group<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>copyUsingHardlinks</b></td>
        <td>boolean</td>
        <td>
          Use hardlinks instead of copy when possible<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>createEmptySeriesFolders</b></td>
        <td>boolean</td>
        <td>
          Create empty series folders during disk scan<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>deleteEmptyFolders</b></td>
        <td>boolean</td>
        <td>
          Delete empty series and season folders during disk scan<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>downloadPropersAndRepacks</b></td>
        <td>enum</td>
        <td>
          Download propers and repacks: DoNotPrefer, PreferAndUpgrade, DoNotUpgrade<br/>
          <br/>
            <i>Enum</i>: DoNotPrefer, PreferAndUpgrade, DoNotUpgrade<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enableMediaInfo</b></td>
        <td>boolean</td>
        <td>
          Enable media info scanning<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>episodeTitleRequired</b></td>
        <td>enum</td>
        <td>
          Episode title required: Always, BulkSeasonReleases, Never<br/>
          <br/>
            <i>Enum</i>: Always, BulkSeasonReleases, Never<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>extraFileExtensions</b></td>
        <td>string</td>
        <td>
          Extra file extensions to import (e.g., "srt,sub")<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>fileDate</b></td>
        <td>enum</td>
        <td>
          File date to use: None, LocalAirDate, UtcAirDate<br/>
          <br/>
            <i>Enum</i>: None, LocalAirDate, UtcAirDate<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>importExtraFiles</b></td>
        <td>boolean</td>
        <td>
          Import extra files (subtitles, etc.)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>minimumFreeSpaceWhenImporting</b></td>
        <td>integer</td>
        <td>
          Minimum free space when importing (MB)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>recycleBin</b></td>
        <td>string</td>
        <td>
          Recycle bin path (empty to disable)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>recycleBinCleanupDays</b></td>
        <td>integer</td>
        <td>
          Days to keep files in recycle bin before cleaning (0 to disable)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>rescanAfterRefresh</b></td>
        <td>enum</td>
        <td>
          Rescan series folder after refresh: Always, AfterManual, Never<br/>
          <br/>
            <i>Enum</i>: Always, AfterManual, Never<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>scriptImportPath</b></td>
        <td>string</td>
        <td>
          Script import path<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>setPermissionsLinux</b></td>
        <td>boolean</td>
        <td>
          Set permissions on Linux/macOS<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>skipFreeSpaceCheckWhenImporting</b></td>
        <td>boolean</td>
        <td>
          Skip free space check when importing<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>useScriptImport</b></td>
        <td>boolean</td>
        <td>
          Use script for importing<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMediaManagementConfig.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrmediamanagementconfigspec)</sup></sup>



Reference to the Sonarr instance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMediaManagementConfig.status
<sup><sup>[↩ Parent](#sonarrmediamanagementconfig)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrmediamanagementconfigstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMediaManagementConfig.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrmediamanagementconfigstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrMetadata
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrMetadataSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrMetadata</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrmetadataspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrMetadata represents a metadata consumer configuration in Sonarr
Metadata consumers write metadata files for media managers (Kodi, Plex, etc.)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrmetadatastatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMetadata.spec
<sup><sup>[↩ Parent](#sonarrmetadata)</sup></sup>



SonarrMetadata represents a metadata consumer configuration in Sonarr
Metadata consumers write metadata files for media managers (Kodi, Plex, etc.)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>metadataType</b></td>
        <td>enum</td>
        <td>
          Metadata type/implementation<br/>
          <br/>
            <i>Enum</i>: xbmcMetadata, roksboxMetadata, wdtvMetadata<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Metadata consumer name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrmetadataspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrmetadataspecconfig">config</a></b></td>
        <td>object</td>
        <td>
          Metadata-specific configuration<br/>
          <br/>
            <i>Default</i>: map[episodeImages:false episodeMetadata:false seasonImages:false seriesImages:false seriesMetadata:false seriesMetadataUrl:false]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enable</b></td>
        <td>boolean</td>
        <td>
          Enable this metadata consumer<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags for this metadata consumer<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMetadata.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrmetadataspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMetadata.spec.config
<sup><sup>[↩ Parent](#sonarrmetadataspec)</sup></sup>



Metadata-specific configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>episodeImages</b></td>
        <td>boolean</td>
        <td>
          Write episode images (thumbnails)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>episodeMetadata</b></td>
        <td>boolean</td>
        <td>
          Write episode metadata (episode.nfo)<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seasonImages</b></td>
        <td>boolean</td>
        <td>
          Write season images<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seriesImages</b></td>
        <td>boolean</td>
        <td>
          Write series images (poster, banner, fanart)<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seriesMetadata</b></td>
        <td>boolean</td>
        <td>
          Write series metadata (series.nfo)<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seriesMetadataUrl</b></td>
        <td>boolean</td>
        <td>
          Write series metadata URL (deprecated)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMetadata.status
<sup><sup>[↩ Parent](#sonarrmetadata)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrmetadatastatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Metadata ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrMetadata.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrmetadatastatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrNamingConfig
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrNamingConfigSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrNamingConfig</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrnamingconfigspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrNamingConfig configures episode naming settings for a Sonarr instance.
Only one SonarrNamingConfig per Sonarr instance is allowed.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrnamingconfigstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNamingConfig.spec
<sup><sup>[↩ Parent](#sonarrnamingconfig)</sup></sup>



SonarrNamingConfig configures episode naming settings for a Sonarr instance.
Only one SonarrNamingConfig per Sonarr instance is allowed.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrnamingconfigspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the Sonarr instance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>animeEpisodeFormat</b></td>
        <td>string</td>
        <td>
          Anime episode format
Example: "{Series Title} - S{season:00}E{episode:00} - {Episode Title} {Quality Full}"<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>colonReplacementFormat</b></td>
        <td>integer</td>
        <td>
          Colon replacement format (0=Delete, 1=Dash, 2=SpaceDash, 3=SpaceDashSpace, 4=Smart)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>customColonReplacementFormat</b></td>
        <td>string</td>
        <td>
          Custom colon replacement format string<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>dailyEpisodeFormat</b></td>
        <td>string</td>
        <td>
          Daily episode format
Example: "{Series Title} - {Air-Date} - {Episode Title} {Quality Full}"<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>multiEpisodeStyle</b></td>
        <td>integer</td>
        <td>
          Multi-episode style (0=Extend, 1=Duplicate, 2=Repeat, 3=Scene, 4=Range, 5=PrefixedRange)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>renameEpisodes</b></td>
        <td>boolean</td>
        <td>
          Enable episode renaming<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>replaceIllegalCharacters</b></td>
        <td>boolean</td>
        <td>
          Replace illegal characters in filenames<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seasonFolderFormat</b></td>
        <td>string</td>
        <td>
          Season folder format
Example: "Season {season}"<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seriesFolderFormat</b></td>
        <td>string</td>
        <td>
          Series folder format
Example: "{Series Title}"<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>specialsFolderFormat</b></td>
        <td>string</td>
        <td>
          Specials folder format
Example: "Specials"<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>standardEpisodeFormat</b></td>
        <td>string</td>
        <td>
          Standard episode format
Example: "{Series Title} - S{season:00}E{episode:00} - {Episode Title} {Quality Full}"<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNamingConfig.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrnamingconfigspec)</sup></sup>



Reference to the Sonarr instance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNamingConfig.status
<sup><sup>[↩ Parent](#sonarrnamingconfig)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrnamingconfigstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNamingConfig.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrnamingconfigstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrNotification
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrNotificationSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrNotification</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrNotification represents a notification/connect configuration in Sonarr
Notifications are used to alert on events (Discord, Telegram, Webhook, etc.)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNotification.spec
<sup><sup>[↩ Parent](#sonarrnotification)</sup></sup>



SonarrNotification represents a notification/connect configuration in Sonarr
Notifications are used to alert on events (Discord, Telegram, Webhook, etc.)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrnotificationspecconfig">config</a></b></td>
        <td>object</td>
        <td>
          Notification configuration<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Notification name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>notificationType</b></td>
        <td>enum</td>
        <td>
          Notification type<br/>
          <br/>
            <i>Enum</i>: Apprise, CustomScript, Discord, Email, Emby, Gotify, Join, Kodi, Mailgun, Ntfy, Plex, Prowl, Pushbullet, Pushover, SendGrid, Signal, Simplepush, Slack, SynologyIndexer, Telegram, Trakt, Twitter, Webhook<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags for this notification<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspectriggers">triggers</a></b></td>
        <td>object</td>
        <td>
          Event triggers<br/>
          <br/>
            <i>Default</i>: map[includeHealthWarnings:false onApplicationUpdate:false onDownload:false onEpisodeFileDelete:false onEpisodeFileDeleteForUpgrade:false onGrab:false onHealthIssue:false onHealthRestored:false onImportComplete:false onManualInteractionRequired:false onRename:false onSeriesAdd:false onSeriesDelete:false onUpgrade:false]<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.config
<sup><sup>[↩ Parent](#sonarrnotificationspec)</sup></sup>



Notification configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrnotificationspecconfigapikeysecretref">apiKeySecretRef</a></b></td>
        <td>object</td>
        <td>
          API key secret reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspecconfigapptokensecretref">appTokenSecretRef</a></b></td>
        <td>object</td>
        <td>
          Gotify app token secret reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>arguments</b></td>
        <td>string</td>
        <td>
          Script arguments<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspecconfigauthtokensecretref">authTokenSecretRef</a></b></td>
        <td>object</td>
        <td>
          Auth token secret reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>avatar</b></td>
        <td>string</td>
        <td>
          Discord avatar<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>bcc</b></td>
        <td>[]string</td>
        <td>
          BCC addresses<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspecconfigbottokensecretref">botTokenSecretRef</a></b></td>
        <td>object</td>
        <td>
          Telegram bot token secret reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>cc</b></td>
        <td>[]string</td>
        <td>
          CC addresses<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>channel</b></td>
        <td>string</td>
        <td>
          Slack channel<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>chatId</b></td>
        <td>string</td>
        <td>
          Telegram chat ID<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>clickUrl</b></td>
        <td>string</td>
        <td>
          Click URL<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>devices</b></td>
        <td>[]string</td>
        <td>
          Device list<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>discordUsername</b></td>
        <td>string</td>
        <td>
          Discord username<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>expire</b></td>
        <td>integer</td>
        <td>
          Expire after (seconds)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>from</b></td>
        <td>string</td>
        <td>
          From address<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>host</b></td>
        <td>string</td>
        <td>
          Server host<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>icon</b></td>
        <td>string</td>
        <td>
          Slack icon<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>mapTo</b></td>
        <td>string</td>
        <td>
          Notify on specific library sections<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>method</b></td>
        <td>integer</td>
        <td>
          HTTP Method (1 = POST, 2 = PUT)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>ntfyTags</b></td>
        <td>[]string</td>
        <td>
          Ntfy tags<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspecconfigpasswordsecretref">passwordSecretRef</a></b></td>
        <td>object</td>
        <td>
          Password secret reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          Path to script<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>port</b></td>
        <td>integer</td>
        <td>
          SMTP port<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>priority</b></td>
        <td>integer</td>
        <td>
          Priority level<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>requireEncryption</b></td>
        <td>boolean</td>
        <td>
          Require encryption<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>retry</b></td>
        <td>integer</td>
        <td>
          Retry interval (seconds)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>sendSilently</b></td>
        <td>boolean</td>
        <td>
          Send silently<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>server</b></td>
        <td>string</td>
        <td>
          SMTP server<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>serverUrl</b></td>
        <td>string</td>
        <td>
          Ntfy server URL<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>slackWebhookUrl</b></td>
        <td>string</td>
        <td>
          Slack webhook URL<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>sound</b></td>
        <td>string</td>
        <td>
          Sound<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>to</b></td>
        <td>[]string</td>
        <td>
          To addresses<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>topic</b></td>
        <td>string</td>
        <td>
          Ntfy topic<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>updateLibrary</b></td>
        <td>boolean</td>
        <td>
          Update library<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>url</b></td>
        <td>string</td>
        <td>
          Webhook URL<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>useSsl</b></td>
        <td>boolean</td>
        <td>
          Use SSL<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrnotificationspecconfiguserkeysecretref">userKeySecretRef</a></b></td>
        <td>object</td>
        <td>
          User key secret reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>username</b></td>
        <td>string</td>
        <td>
          Username for basic auth<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>webhookUrl</b></td>
        <td>string</td>
        <td>
          Discord webhook URL<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.config.apiKeySecretRef
<sup><sup>[↩ Parent](#sonarrnotificationspecconfig)</sup></sup>



API key secret reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.config.appTokenSecretRef
<sup><sup>[↩ Parent](#sonarrnotificationspecconfig)</sup></sup>



Gotify app token secret reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.config.authTokenSecretRef
<sup><sup>[↩ Parent](#sonarrnotificationspecconfig)</sup></sup>



Auth token secret reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.config.botTokenSecretRef
<sup><sup>[↩ Parent](#sonarrnotificationspecconfig)</sup></sup>



Telegram bot token secret reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.config.passwordSecretRef
<sup><sup>[↩ Parent](#sonarrnotificationspecconfig)</sup></sup>



Password secret reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.config.userKeySecretRef
<sup><sup>[↩ Parent](#sonarrnotificationspecconfig)</sup></sup>



User key secret reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrnotificationspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNotification.spec.triggers
<sup><sup>[↩ Parent](#sonarrnotificationspec)</sup></sup>



Event triggers

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>includeHealthWarnings</b></td>
        <td>boolean</td>
        <td>
          Include health warnings<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onApplicationUpdate</b></td>
        <td>boolean</td>
        <td>
          On application update<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onDownload</b></td>
        <td>boolean</td>
        <td>
          On download (episode is downloaded)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onEpisodeFileDelete</b></td>
        <td>boolean</td>
        <td>
          On episode file delete<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onEpisodeFileDeleteForUpgrade</b></td>
        <td>boolean</td>
        <td>
          On episode file delete for upgrade<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onGrab</b></td>
        <td>boolean</td>
        <td>
          On grab (episode is grabbed)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onHealthIssue</b></td>
        <td>boolean</td>
        <td>
          On health issue<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onHealthRestored</b></td>
        <td>boolean</td>
        <td>
          On health restored<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onImportComplete</b></td>
        <td>boolean</td>
        <td>
          On import complete<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onManualInteractionRequired</b></td>
        <td>boolean</td>
        <td>
          On manual interaction required<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onRename</b></td>
        <td>boolean</td>
        <td>
          On rename<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onSeriesAdd</b></td>
        <td>boolean</td>
        <td>
          On series add<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onSeriesDelete</b></td>
        <td>boolean</td>
        <td>
          On series delete<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>onUpgrade</b></td>
        <td>boolean</td>
        <td>
          On upgrade (episode is upgraded)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNotification.status
<sup><sup>[↩ Parent](#sonarrnotification)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrnotificationstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Notification ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrNotification.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrnotificationstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrQualityDefinition
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrQualityDefinitionSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrQualityDefinition</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrqualitydefinitionspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrQualityDefinition represents a quality definition configuration in Sonarr
Quality definitions control the size limits for each quality level<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrqualitydefinitionstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityDefinition.spec
<sup><sup>[↩ Parent](#sonarrqualitydefinition)</sup></sup>



SonarrQualityDefinition represents a quality definition configuration in Sonarr
Quality definitions control the size limits for each quality level

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>qualityName</b></td>
        <td>enum</td>
        <td>
          Quality name (must match existing quality in Sonarr)<br/>
          <br/>
            <i>Enum</i>: UNKNOWN, SDTV, DVD, WEBDL-480p, WEBRip-480p, Bluray-480p, HDTV-720p, HDTV-1080p, Raw-HD, WEBDL-720p, WEBRip-720p, Bluray-720p, WEBDL-1080p, WEBRip-1080p, Bluray-1080p, Bluray-1080p Remux, HDTV-2160p, WEBDL-2160p, WEBRip-2160p, Bluray-2160p, Bluray-2160p Remux<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrqualitydefinitionspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>maxSize</b></td>
        <td>number</td>
        <td>
          Maximum size in MB per minute of runtime (None = unlimited)<br/>
          <br/>
            <i>Format</i>: double<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>minSize</b></td>
        <td>number</td>
        <td>
          Minimum size in MB per minute of runtime<br/>
          <br/>
            <i>Format</i>: double<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>preferredSize</b></td>
        <td>number</td>
        <td>
          Preferred size in MB per minute of runtime<br/>
          <br/>
            <i>Format</i>: double<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>title</b></td>
        <td>string</td>
        <td>
          Title/display name for this quality<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityDefinition.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrqualitydefinitionspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityDefinition.status
<sup><sup>[↩ Parent](#sonarrqualitydefinition)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrqualitydefinitionstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Quality Definition ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityDefinition.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrqualitydefinitionstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrQualityProfile
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrQualityProfileSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrQualityProfile</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrqualityprofilespec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrQualityProfile represents a quality profile in Sonarr
Quality profiles define which qualities are acceptable and their priority<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrqualityprofilestatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityProfile.spec
<sup><sup>[↩ Parent](#sonarrqualityprofile)</sup></sup>



SonarrQualityProfile represents a quality profile in Sonarr
Quality profiles define which qualities are acceptable and their priority

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Quality profile name<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrqualityprofilespecqualitygroupsindex">qualityGroups</a></b></td>
        <td>[]object</td>
        <td>
          Ordered list of quality groups<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrqualityprofilespecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>cutoff</b></td>
        <td>integer</td>
        <td>
          Quality ID to use as cutoff<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>cutoffFormatScore</b></td>
        <td>integer</td>
        <td>
          Cutoff format score<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrqualityprofilespecformatitemsindex">formatItems</a></b></td>
        <td>[]object</td>
        <td>
          Format items (custom formats with scores)<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>minFormatScore</b></td>
        <td>integer</td>
        <td>
          Minimum format score<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>minUpgradeFormatScore</b></td>
        <td>integer</td>
        <td>
          Minimum upgrade format score<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>upgradeAllowed</b></td>
        <td>boolean</td>
        <td>
          Whether upgrades are allowed<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityProfile.spec.qualityGroups[index]
<sup><sup>[↩ Parent](#sonarrqualityprofilespec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrqualityprofilespecqualitygroupsindexqualitiesindex">qualities</a></b></td>
        <td>[]object</td>
        <td>
          Ordered list of qualities in this group<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Quality group ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Quality group name<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityProfile.spec.qualityGroups[index].qualities[index]
<sup><sup>[↩ Parent](#sonarrqualityprofilespecqualitygroupsindex)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Quality ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Quality name<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>resolution</b></td>
        <td>integer</td>
        <td>
          Resolution<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>source</b></td>
        <td>string</td>
        <td>
          Source type<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityProfile.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrqualityprofilespec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityProfile.spec.formatItems[index]
<sup><sup>[↩ Parent](#sonarrqualityprofilespec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>format</b></td>
        <td>integer</td>
        <td>
          Custom format ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Format name<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>score</b></td>
        <td>integer</td>
        <td>
          Score for this format<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityProfile.status
<sup><sup>[↩ Parent](#sonarrqualityprofile)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrqualityprofilestatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Quality Profile ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrQualityProfile.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrqualityprofilestatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrRootFolder
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrRootFolderSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrRootFolder</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrrootfolderspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrRootFolder represents a root folder in Sonarr
Root folders are the base directories where series are stored<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrrootfolderstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrRootFolder.spec
<sup><sup>[↩ Parent](#sonarrrootfolder)</sup></sup>



SonarrRootFolder represents a root folder in Sonarr
Root folders are the base directories where series are stored

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          Root folder absolute path<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrrootfolderspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrRootFolder.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrrootfolderspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrRootFolder.status
<sup><sup>[↩ Parent](#sonarrrootfolder)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>accessible</b></td>
        <td>boolean</td>
        <td>
          Whether the folder is accessible<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrrootfolderstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>freeSpace</b></td>
        <td>integer</td>
        <td>
          Free space in the folder<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Root Folder ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrRootFolder.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrrootfolderstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrSeries
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrSeriesSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrSeries</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrseriesspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrSeries represents a TV series managed in Sonarr
This allows declarative management of series in your library<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrseriesstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrSeries.spec
<sup><sup>[↩ Parent](#sonarrseries)</sup></sup>



SonarrSeries represents a TV series managed in Sonarr
This allows declarative management of series in your library

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrseriesspecqualityprofile">qualityProfile</a></b></td>
        <td>object</td>
        <td>
          Quality profile ID or name reference<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>rootFolderPath</b></td>
        <td>string</td>
        <td>
          Root folder path for the series<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrseriesspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>title</b></td>
        <td>string</td>
        <td>
          Series title<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>titleSlug</b></td>
        <td>string</td>
        <td>
          Title slug (kebab-case version of title)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>tvdbId</b></td>
        <td>integer</td>
        <td>
          TVDB ID for the series<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrseriesspecaddoptions">addOptions</a></b></td>
        <td>object</td>
        <td>
          Monitor type for adding series<br/>
          <br/>
            <i>Default</i>: map[monitor:all searchForCutoffUnmetEpisodes:false searchForMissingEpisodes:true]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>monitored</b></td>
        <td>boolean</td>
        <td>
          Whether the series is monitored<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          Specific path override (optional)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seasonFolder</b></td>
        <td>boolean</td>
        <td>
          Use season folders<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seriesType</b></td>
        <td>enum</td>
        <td>
          Series type<br/>
          <br/>
            <i>Enum</i>: standard, daily, anime<br/>
            <i>Default</i>: standard<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tags</b></td>
        <td>[]integer</td>
        <td>
          Tags for this series<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>useSceneNumbering</b></td>
        <td>boolean</td>
        <td>
          Use scene numbering<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrSeries.spec.qualityProfile
<sup><sup>[↩ Parent](#sonarrseriesspec)</sup></sup>



Quality profile ID or name reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Quality profile ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Quality profile name (will be resolved to ID)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrSeries.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrseriesspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrSeries.spec.addOptions
<sup><sup>[↩ Parent](#sonarrseriesspec)</sup></sup>



Monitor type for adding series

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>monitor</b></td>
        <td>enum</td>
        <td>
          Monitor type<br/>
          <br/>
            <i>Enum</i>: all, future, missing, existing, recent, pilot, firstseason, lastseason, none<br/>
            <i>Default</i>: all<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>searchForCutoffUnmetEpisodes</b></td>
        <td>boolean</td>
        <td>
          Search for cutoff unmet episodes<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>searchForMissingEpisodes</b></td>
        <td>boolean</td>
        <td>
          Search for missing episodes when adding<br/>
          <br/>
            <i>Default</i>: true<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrSeries.status
<sup><sup>[↩ Parent](#sonarrseries)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrseriesstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>episodeCount</b></td>
        <td>integer</td>
        <td>
          Total episode count<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>episodeFileCount</b></td>
        <td>integer</td>
        <td>
          Episode file count<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Series ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>network</b></td>
        <td>string</td>
        <td>
          Network<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>nextAiring</b></td>
        <td>string</td>
        <td>
          Next airing date<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>percentComplete</b></td>
        <td>number</td>
        <td>
          Percentage complete<br/>
          <br/>
            <i>Format</i>: double<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>previousAiring</b></td>
        <td>string</td>
        <td>
          Previous airing date<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>seriesStatus</b></td>
        <td>string</td>
        <td>
          Status (continuing, ended, etc.)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrSeries.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrseriesstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## Sonarr
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>Sonarr</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrspec">spec</a></b></td>
        <td>object</td>
        <td>
          Sonarr is the main CRD that deploys and manages a Sonarr instance

This CRD creates:
- A Deployment with the Sonarr container
- An init container for database migrations
- A Service to expose Sonarr
- A PersistentVolumeClaim for configuration storage
- Optional Ingress for external access<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec
<sup><sup>[↩ Parent](#sonarr)</sup></sup>



Sonarr is the main CRD that deploys and manages a Sonarr instance

This CRD creates:
- A Deployment with the Sonarr container
- An init container for database migrations
- A Service to expose Sonarr
- A PersistentVolumeClaim for configuration storage
- Optional Ingress for external access

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrspecapikeysecretref">apiKeySecretRef</a></b></td>
        <td>object</td>
        <td>
          API key secret reference (optional - will be auto-generated if not provided)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecconfig">config</a></b></td>
        <td>object</td>
        <td>
          Sonarr application configuration (config.xml settings)<br/>
          <br/>
            <i>Default</i>: map[analyticsEnabled:<nil> authenticationMethod:<nil> authenticationRequired:<nil> bindAddress:<nil> initContainerImage:<nil> instanceName:<nil> logLevel:<nil> urlBase:<nil>]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecenvindex">env</a></b></td>
        <td>[]object</td>
        <td>
          Environment variables<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspechttproute">httpRoute</a></b></td>
        <td>object</td>
        <td>
          HTTPRoute configuration for Gateway API (optional)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>image</b></td>
        <td>string</td>
        <td>
          Sonarr image to use (default: lscr.io/linuxserver/sonarr:latest)<br/>
          <br/>
            <i>Default</i>: lscr.io/linuxserver/sonarr:latest<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>imagePullPolicy</b></td>
        <td>string</td>
        <td>
          Image pull policy (default: IfNotPresent)<br/>
          <br/>
            <i>Default</i>: IfNotPresent<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecingress">ingress</a></b></td>
        <td>object</td>
        <td>
          Ingress configuration (optional)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecinitcontainer">initContainer</a></b></td>
        <td>object</td>
        <td>
          Init container configuration (for custom init logic)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>nodeSelector</b></td>
        <td>map[string]string</td>
        <td>
          Node selector<br/>
          <br/>
            <i>Default</i>: map[]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>replicas</b></td>
        <td>integer</td>
        <td>
          Number of replicas (should be 1 for Sonarr)<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 1<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecresources">resources</a></b></td>
        <td>object</td>
        <td>
          Resource requirements<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecsecuritycontext">securityContext</a></b></td>
        <td>object</td>
        <td>
          Pod security context<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecservice">service</a></b></td>
        <td>object</td>
        <td>
          Service configuration<br/>
          <br/>
            <i>Default</i>: map[annotations:map[] containerPort:0 nodePort:<nil> port:0 serviceType:]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecstorage">storage</a></b></td>
        <td>object</td>
        <td>
          Storage configuration<br/>
          <br/>
            <i>Default</i>: map[accessModes:[] existingClaim:<nil> size: storageClass:<nil>]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspectolerationsindex">tolerations</a></b></td>
        <td>[]object</td>
        <td>
          Tolerations<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumemountsindex">volumeMounts</a></b></td>
        <td>[]object</td>
        <td>
          Volume mounts for media directories<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumesindex">volumes</a></b></td>
        <td>[]object</td>
        <td>
          Additional volumes<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.apiKeySecretRef
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



API key secret reference (optional - will be auto-generated if not provided)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### Sonarr.spec.config
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



Sonarr application configuration (config.xml settings)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>analyticsEnabled</b></td>
        <td>boolean</td>
        <td>
          Analytics enabled (default: true)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>authenticationMethod</b></td>
        <td>string</td>
        <td>
          Authentication method: None, Basic, Forms, External (default: None)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>authenticationRequired</b></td>
        <td>boolean</td>
        <td>
          Authentication required for API access (default: false)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>bindAddress</b></td>
        <td>string</td>
        <td>
          Bind address (default: "*")<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>initContainerImage</b></td>
        <td>string</td>
        <td>
          Init container image used to configure config.xml (default: busybox:latest)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>instanceName</b></td>
        <td>string</td>
        <td>
          Instance name displayed in the UI<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>logLevel</b></td>
        <td>string</td>
        <td>
          Log level: trace, debug, info, warn, error (default: info)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>urlBase</b></td>
        <td>string</td>
        <td>
          URL base for reverse proxy setups (e.g., "/sonarr")<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.env[index]
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the environment variable<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>value</b></td>
        <td>string</td>
        <td>
          Value of the environment variable<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecenvindexvaluefrom">valueFrom</a></b></td>
        <td>object</td>
        <td>
          Reference to a secret or configmap<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.env[index].valueFrom
<sup><sup>[↩ Parent](#sonarrspecenvindex)</sup></sup>



Reference to a secret or configmap

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrspecenvindexvaluefromconfigmapkeyref">configMapKeyRef</a></b></td>
        <td>object</td>
        <td>
          ConfigMap key reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecenvindexvaluefromsecretkeyref">secretKeyRef</a></b></td>
        <td>object</td>
        <td>
          Secret key reference<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.env[index].valueFrom.configMapKeyRef
<sup><sup>[↩ Parent](#sonarrspecenvindexvaluefrom)</sup></sup>



ConfigMap key reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the configmap<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the configmap<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### Sonarr.spec.env[index].valueFrom.secretKeyRef
<sup><sup>[↩ Parent](#sonarrspecenvindexvaluefrom)</sup></sup>



Secret key reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### Sonarr.spec.httpRoute
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



HTTPRoute configuration for Gateway API (optional)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrspechttproutegatewayref">gatewayRef</a></b></td>
        <td>object</td>
        <td>
          Gateway reference - the Gateway to attach to<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>annotations</b></td>
        <td>map[string]string</td>
        <td>
          Additional annotations for the HTTPRoute<br/>
          <br/>
            <i>Default</i>: map[]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enabled</b></td>
        <td>boolean</td>
        <td>
          Enable HTTPRoute creation (default: false)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>hostnames</b></td>
        <td>[]string</td>
        <td>
          Hostnames for the HTTPRoute<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>labels</b></td>
        <td>map[string]string</td>
        <td>
          Additional labels for the HTTPRoute<br/>
          <br/>
            <i>Default</i>: map[]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          Path match for the route (default: /)<br/>
          <br/>
            <i>Default</i>: /<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>pathType</b></td>
        <td>string</td>
        <td>
          Path match type: Exact, PathPrefix, or RegularExpression (default: PathPrefix)<br/>
          <br/>
            <i>Default</i>: PathPrefix<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.httpRoute.gatewayRef
<sup><sup>[↩ Parent](#sonarrspechttproute)</sup></sup>



Gateway reference - the Gateway to attach to

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the Gateway<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the Gateway (optional, defaults to same namespace as HTTPRoute)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>sectionName</b></td>
        <td>string</td>
        <td>
          Section name within the Gateway (optional)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.ingress
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



Ingress configuration (optional)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>host</b></td>
        <td>string</td>
        <td>
          Hostname for the ingress<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>annotations</b></td>
        <td>map[string]string</td>
        <td>
          Ingress annotations<br/>
          <br/>
            <i>Default</i>: map[]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>enabled</b></td>
        <td>boolean</td>
        <td>
          Enable ingress (default: false)<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>ingressClassName</b></td>
        <td>string</td>
        <td>
          Ingress class name<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          Path for the ingress (default: /)<br/>
          <br/>
            <i>Default</i>: /<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>pathType</b></td>
        <td>string</td>
        <td>
          Path type (default: Prefix)<br/>
          <br/>
            <i>Default</i>: Prefix<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecingresstls">tls</a></b></td>
        <td>object</td>
        <td>
          TLS configuration<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.ingress.tls
<sup><sup>[↩ Parent](#sonarrspecingress)</sup></sup>



TLS configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>secretName</b></td>
        <td>string</td>
        <td>
          Secret name containing TLS certificate<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>hosts</b></td>
        <td>[]string</td>
        <td>
          Hosts covered by the TLS certificate<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.initContainer
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



Init container configuration (for custom init logic)

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>args</b></td>
        <td>[]string</td>
        <td>
          Arguments for the command<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>command</b></td>
        <td>[]string</td>
        <td>
          Command to run in init container<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecinitcontainerenvindex">env</a></b></td>
        <td>[]object</td>
        <td>
          Environment variables for init container<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>image</b></td>
        <td>string</td>
        <td>
          Image for init container (default: busybox:latest)<br/>
          <br/>
            <i>Default</i>: busybox:latest<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.initContainer.env[index]
<sup><sup>[↩ Parent](#sonarrspecinitcontainer)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the environment variable<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>value</b></td>
        <td>string</td>
        <td>
          Value of the environment variable<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecinitcontainerenvindexvaluefrom">valueFrom</a></b></td>
        <td>object</td>
        <td>
          Reference to a secret or configmap<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.initContainer.env[index].valueFrom
<sup><sup>[↩ Parent](#sonarrspecinitcontainerenvindex)</sup></sup>



Reference to a secret or configmap

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrspecinitcontainerenvindexvaluefromconfigmapkeyref">configMapKeyRef</a></b></td>
        <td>object</td>
        <td>
          ConfigMap key reference<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecinitcontainerenvindexvaluefromsecretkeyref">secretKeyRef</a></b></td>
        <td>object</td>
        <td>
          Secret key reference<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.initContainer.env[index].valueFrom.configMapKeyRef
<sup><sup>[↩ Parent](#sonarrspecinitcontainerenvindexvaluefrom)</sup></sup>



ConfigMap key reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the configmap<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the configmap<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### Sonarr.spec.initContainer.env[index].valueFrom.secretKeyRef
<sup><sup>[↩ Parent](#sonarrspecinitcontainerenvindexvaluefrom)</sup></sup>



Secret key reference

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          Key in the secret<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the secret<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### Sonarr.spec.resources
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



Resource requirements

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>limits</b></td>
        <td>map[string]string</td>
        <td>
          Resource limits<br/>
          <br/>
            <i>Default</i>: map[]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>requests</b></td>
        <td>map[string]string</td>
        <td>
          Resource requests<br/>
          <br/>
            <i>Default</i>: map[]<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.securityContext
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



Pod security context

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>fsGroup</b></td>
        <td>integer</td>
        <td>
          <br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>runAsGroup</b></td>
        <td>integer</td>
        <td>
          <br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>runAsNonRoot</b></td>
        <td>boolean</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>runAsUser</b></td>
        <td>integer</td>
        <td>
          <br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.service
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



Service configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>annotations</b></td>
        <td>map[string]string</td>
        <td>
          Service annotations<br/>
          <br/>
            <i>Default</i>: map[]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>containerPort</b></td>
        <td>integer</td>
        <td>
          Container port - the port Sonarr listens on inside the container (default: 8989)<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 8989<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>nodePort</b></td>
        <td>integer</td>
        <td>
          Node port (only for NodePort type)<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>port</b></td>
        <td>integer</td>
        <td>
          Service port (default: 8989)<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 8989<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>serviceType</b></td>
        <td>string</td>
        <td>
          Service type (default: ClusterIP)<br/>
          <br/>
            <i>Default</i>: ClusterIP<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.storage
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>



Storage configuration

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>accessModes</b></td>
        <td>[]string</td>
        <td>
          Access modes (default: ReadWriteOnce)<br/>
          <br/>
            <i>Default</i>: [ReadWriteOnce]<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>existingClaim</b></td>
        <td>string</td>
        <td>
          Existing PVC to use (optional)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>size</b></td>
        <td>string</td>
        <td>
          Size of the config PVC (default: 1Gi)<br/>
          <br/>
            <i>Default</i>: 1Gi<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>storageClass</b></td>
        <td>string</td>
        <td>
          Storage class for the PVC<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.tolerations[index]
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>effect</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>operator</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>tolerationSeconds</b></td>
        <td>integer</td>
        <td>
          <br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>value</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumeMounts[index]
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>mountPath</b></td>
        <td>string</td>
        <td>
          Mount path inside the container<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the volume<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>readOnly</b></td>
        <td>boolean</td>
        <td>
          Read only flag<br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>subPath</b></td>
        <td>string</td>
        <td>
          Sub path (optional)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumes[index]
<sup><sup>[↩ Parent](#sonarrspec)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the volume<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumesindexconfigmap">configMap</a></b></td>
        <td>object</td>
        <td>
          ConfigMap volume<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumesindexemptydir">emptyDir</a></b></td>
        <td>object</td>
        <td>
          Empty dir volume<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumesindexhostpath">hostPath</a></b></td>
        <td>object</td>
        <td>
          HostPath volume<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumesindexnfs">nfs</a></b></td>
        <td>object</td>
        <td>
          NFS volume<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumesindexpersistentvolumeclaim">persistentVolumeClaim</a></b></td>
        <td>object</td>
        <td>
          PVC claim<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumes[index].configMap
<sup><sup>[↩ Parent](#sonarrspecvolumesindex)</sup></sup>



ConfigMap volume

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrspecvolumesindexconfigmapitemsindex">items</a></b></td>
        <td>[]object</td>
        <td>
          <br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumes[index].configMap.items[index]
<sup><sup>[↩ Parent](#sonarrspecvolumesindexconfigmap)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>key</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumes[index].emptyDir
<sup><sup>[↩ Parent](#sonarrspecvolumesindex)</sup></sup>



Empty dir volume

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>medium</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>sizeLimit</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumes[index].hostPath
<sup><sup>[↩ Parent](#sonarrspecvolumesindex)</sup></sup>



HostPath volume

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumes[index].nfs
<sup><sup>[↩ Parent](#sonarrspecvolumesindex)</sup></sup>



NFS volume

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>path</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>server</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>readOnly</b></td>
        <td>boolean</td>
        <td>
          <br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.spec.volumes[index].persistentVolumeClaim
<sup><sup>[↩ Parent](#sonarrspecvolumesindex)</sup></sup>



PVC claim

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>claimName</b></td>
        <td>string</td>
        <td>
          <br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>readOnly</b></td>
        <td>boolean</td>
        <td>
          <br/>
          <br/>
            <i>Default</i>: false<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.status
<sup><sup>[↩ Parent](#sonarr)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>apiKeySecret</b></td>
        <td>string</td>
        <td>
          API key (stored in secret)<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b><a href="#sonarrstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>readyReplicas</b></td>
        <td>integer</td>
        <td>
          Number of ready replicas<br/>
          <br/>
            <i>Format</i>: int32<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>url</b></td>
        <td>string</td>
        <td>
          URL to access Sonarr<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>version</b></td>
        <td>string</td>
        <td>
          Sonarr version<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### Sonarr.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>

## SonarrTag
<sup><sup>[↩ Parent](#devopsarriov1alpha1 )</sup></sup>






Auto-generated derived type for SonarrTagSpec via `CustomResource`

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
      <td><b>apiVersion</b></td>
      <td>string</td>
      <td>devopsarr.io/v1alpha1</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b>kind</b></td>
      <td>string</td>
      <td>SonarrTag</td>
      <td>true</td>
      </tr>
      <tr>
      <td><b><a href="https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.27/#objectmeta-v1-meta">metadata</a></b></td>
      <td>object</td>
      <td>Refer to the Kubernetes API documentation for the fields of the `metadata` field.</td>
      <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrtagspec">spec</a></b></td>
        <td>object</td>
        <td>
          SonarrTag represents a tag in Sonarr
Tags are used to organize and filter series, profiles, and other resources<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrtagstatus">status</a></b></td>
        <td>object</td>
        <td>
          <br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrTag.spec
<sup><sup>[↩ Parent](#sonarrtag)</sup></sup>



SonarrTag represents a tag in Sonarr
Tags are used to organize and filter series, profiles, and other resources

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>label</b></td>
        <td>string</td>
        <td>
          Tag label (must be lowercase)<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b><a href="#sonarrtagspecsonarrinstanceref">sonarrInstanceRef</a></b></td>
        <td>object</td>
        <td>
          Reference to the SonarrInstance<br/>
        </td>
        <td>true</td>
      </tr></tbody>
</table>


### SonarrTag.spec.sonarrInstanceRef
<sup><sup>[↩ Parent](#sonarrtagspec)</sup></sup>



Reference to the SonarrInstance

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>name</b></td>
        <td>string</td>
        <td>
          Name of the SonarrInstance resource<br/>
          <br/>
            <i>Default</i>: <br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>namespace</b></td>
        <td>string</td>
        <td>
          Namespace of the SonarrInstance (optional, defaults to same namespace)<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrTag.status
<sup><sup>[↩ Parent](#sonarrtag)</sup></sup>





<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b><a href="#sonarrtagstatusconditionsindex">conditions</a></b></td>
        <td>[]object</td>
        <td>
          Current conditions<br/>
          <br/>
            <i>Default</i>: []<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>id</b></td>
        <td>integer</td>
        <td>
          Sonarr Tag ID<br/>
          <br/>
            <i>Format</i>: int32<br/>
        </td>
        <td>false</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          Observed generation<br/>
          <br/>
            <i>Format</i>: int64<br/>
            <i>Default</i>: 0<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>


### SonarrTag.status.conditions[index]
<sup><sup>[↩ Parent](#sonarrtagstatus)</sup></sup>



Condition contains details for one aspect of the current state of this API Resource.

<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Description</th>
            <th>Required</th>
        </tr>
    </thead>
    <tbody><tr>
        <td><b>lastTransitionTime</b></td>
        <td>string</td>
        <td>
          lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.<br/>
          <br/>
            <i>Format</i>: date-time<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>message</b></td>
        <td>string</td>
        <td>
          message is a human readable message indicating details about the transition. This may be an empty string.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>reason</b></td>
        <td>string</td>
        <td>
          reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>status</b></td>
        <td>string</td>
        <td>
          status of the condition, one of True, False, Unknown.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>type</b></td>
        <td>string</td>
        <td>
          type of condition in CamelCase or in foo.example.com/CamelCase.<br/>
        </td>
        <td>true</td>
      </tr><tr>
        <td><b>observedGeneration</b></td>
        <td>integer</td>
        <td>
          observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.<br/>
          <br/>
            <i>Format</i>: int64<br/>
        </td>
        <td>false</td>
      </tr></tbody>
</table>
