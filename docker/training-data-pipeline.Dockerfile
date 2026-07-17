FROM apache/spark:3.5.7-java17-python3

ARG ICEBERG_VERSION=1.10.0
ARG HADOOP_VERSION=3.3.4
ARG AWS_SDK_VERSION=1.12.262
USER root
RUN set -eu; \
    base=https://repo1.maven.org/maven2; \
    fetch() { curl -fsSL "$1" -o "/opt/spark/jars/$2"; echo "$3  /opt/spark/jars/$2" | sha256sum -c -; }; \
    fetch "$base/org/apache/iceberg/iceberg-spark-runtime-3.5_2.12/$ICEBERG_VERSION/iceberg-spark-runtime-3.5_2.12-$ICEBERG_VERSION.jar" iceberg-runtime.jar 70f9471870b2d456167d703c829f098a446b86bb1379eee338c0efa03a341026; \
    fetch "$base/org/apache/iceberg/iceberg-aws-bundle/$ICEBERG_VERSION/iceberg-aws-bundle-$ICEBERG_VERSION.jar" iceberg-aws.jar 172370cfc32a5f1ae5399b210e86fc04a366dc29a112d97ac8ffc74df7525596; \
    fetch "$base/org/apache/hadoop/hadoop-aws/$HADOOP_VERSION/hadoop-aws-$HADOOP_VERSION.jar" hadoop-aws.jar 53f9ae03c681a30a50aa17524bd9790ab596b28481858e54efd989a826ed3a4a; \
    fetch "$base/com/amazonaws/aws-java-sdk-bundle/$AWS_SDK_VERSION/aws-java-sdk-bundle-$AWS_SDK_VERSION.jar" aws-sdk.jar 873fe7cf495126619997bec21c44de5d992544aea7e632fdc77adb1a0915bae5
WORKDIR /opt/spark/jobs
COPY scripts/clean_training_data_spark.py ./
COPY scripts/submit_training_cleanup_k8s.sh ./
COPY scripts/training_spark_secrets.sh ./
COPY scripts/verify_spark_driver.sh ./
COPY scripts/training_cleanup ./training_cleanup
COPY scripts/training_catalog ./training_catalog
RUN python3 -m zipfile -c training_cleanup.zip training_cleanup
RUN chmod 0555 submit_training_cleanup_k8s.sh training_spark_secrets.sh verify_spark_driver.sh \
 && chown -R 185:0 /opt/spark/jobs \
 && chmod -R g=u /opt/spark/jobs
USER 185
